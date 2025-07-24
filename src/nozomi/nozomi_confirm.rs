use reqwest::Client;
use serde_json::json;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, hash::Hash, instruction::Instruction,
    native_token::sol_to_lamports, pubkey::Pubkey, signature::Keypair, system_instruction,
};
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::{
    HEALTH_CHECK_SEC, JsonRpcError, JsonRpcResponse, NOZOMI_MIN_TIP, NOZOMI_REGIONS, NOZOMI_TIP,
    NozomiEndpoint, NozomiRegionsType, PING_DURATION_SEC, Tips, TransactionBuilder, build_v0_bs64,
    format_elapsed, ping_all, ping_one, simulate,
};

#[derive(Debug)]
pub struct Nozomi {
    pub client: Client,
    pub endpoint: NozomiEndpoint,
    pub auth_key: String,
}

impl TransactionBuilder for Nozomi {
    fn build_v0_bs64(
        &self,
        ixs: Vec<Instruction>,
        fee_payer: &Pubkey,
        signers: &Vec<&Keypair>,
        recent_blockhash: Hash,
        nonce_ix: Option<Instruction>,
    ) -> String {
        build_v0_bs64(ixs, fee_payer, signers, recent_blockhash, nonce_ix)
    }

    fn simulate(
        &self,
        ixs: Vec<Instruction>,
        fee_payer: &Pubkey,
        signers: &Vec<&Keypair>,
        recent_blockhash: Hash,
        nonce_ix: Option<Instruction>,
        rpc_endpoint: String,
    ) {
        simulate(
            ixs,
            fee_payer,
            signers,
            recent_blockhash,
            nonce_ix,
            rpc_endpoint,
        );
    }
}

impl Nozomi {
    pub async fn new_with_region(region: NozomiRegionsType, auth_key: String) -> Self {
        let endpoint = NOZOMI_REGIONS
            .iter()
            .find(|r| r.relayer == region)
            .expect("Region not found")
            .clone();

        // Await the ping
        if let Err(err) = ping_one(
            endpoint.relayer_name.to_string(),
            endpoint.ping_endpoint.to_string(),
            PING_DURATION_SEC,
        )
        .await
        {
            println!("Ping failed during init: {}", err);
        }

        Self {
            client: Client::builder()
                .tcp_keepalive(Duration::from_secs(HEALTH_CHECK_SEC))
                .build()
                .expect("Failed to build Jito HTTP client"),
            endpoint,
            auth_key,
        }
    }

    pub async fn new_auto(auth_key: String) -> Self {
        let regions: Vec<(String, String)> = NOZOMI_REGIONS
            .iter()
            .map(|r| (r.relayer_name.to_string(), r.ping_endpoint.to_string()))
            .collect();

        // Step 1: Ping all regions
        let fastest_index = ping_all(regions, PING_DURATION_SEC).await;

        // Step 2: Use fastest or fallback
        let endpoint = fastest_index
            .map(|i| NOZOMI_REGIONS[i].clone())
            .unwrap_or_else(|| {
                println!("All region pings failed, falling back to first region.");
                NOZOMI_REGIONS[0].clone()
            });

        println!("Connecting with {} ...", endpoint.relayer_name);

        // Optional: Ping chosen one again
        if let Err(err) = ping_one(
            endpoint.relayer_name.to_string(),
            endpoint.ping_endpoint.to_string(),
            2,
        )
        .await
        {
            println!("Ping failed during init: {}", err);
        }

        Self {
            client: Client::builder()
                .tcp_keepalive(Duration::from_secs(HEALTH_CHECK_SEC))
                .build()
                .expect("Failed to build HTTP client"),
            endpoint,
            auth_key,
        }
    }

    pub fn health_check(&self, interval_sec: u64) {
        let client = self.client.clone();
        let endpoint = self.endpoint.clone();
        let relayer_name = self.endpoint.relayer_name.clone(); // Clone this separately

        tokio::spawn(async move {
            let ping_url = format!("https://{}/ping", endpoint.ping_endpoint);

            loop {
                match client.get(&ping_url).send().await {
                    Ok(response) if response.status().is_success() => {
                        println!("{} Health Check Successful", relayer_name);
                    }
                    Ok(response) => {
                        eprintln!(
                            "{} Health Check failed with status: {}",
                            relayer_name,
                            response.status()
                        );
                    }
                    Err(err) => {
                        eprintln!("{} Health Check request error: {:?}", relayer_name, err);
                    }
                }

                sleep(Duration::from_secs(interval_sec)).await;
            }
        });
    }

    pub fn add_tip_ix(&self, tip_config: Tips) -> Vec<Instruction> {
        let mut ixs: Vec<Instruction> = Vec::new();

        if let Some(cu) = tip_config.cu {
            ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(cu as u32));
        };

        if let Some(priority_fee_micro_lamport) = tip_config.priority_fee_micro_lamport {
            ixs.push(ComputeBudgetInstruction::set_compute_unit_price(
                priority_fee_micro_lamport,
            ));
        };

        ixs.extend(tip_config.pure_ix.clone());

        let relayer_fee = tip_config.tip_sol_amount.max(NOZOMI_MIN_TIP); // use `.max()` for clarity

        let recipient = Pubkey::from_str_const(NOZOMI_TIP[tip_config.tip_addr_idx as usize]);
        let transfer_ix = system_instruction::transfer(
            &tip_config.payer,
            &recipient,
            sol_to_lamports(relayer_fee),
        );
        ixs.push(transfer_ix);

        ixs
    }

    pub async fn send_transaction(&self, encoded_tx: &str) -> anyhow::Result<JsonRpcResponse> {
        let start = Instant::now();

        let url = format!("{}{}", self.endpoint.submit_endpoint, self.auth_key);

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [encoded_tx, {"encoding": "base64"}]
        });

        let response = self.client.post(url).json(&payload).send().await?;

        let body = response.text().await?;
        println!("Raw response body:\n{}", body);

        // Parse and return response body as JSON
        let response: JsonRpcResponse = serde_json::from_str(&body).unwrap_or_else(|_| {
            JsonRpcResponse {
                id: 1,
                jsonrpc: "2.0".to_string(), // or "" if you prefer
                result: None,
                error: Some(JsonRpcError {
                    code : 1,
                    message: body.clone(), // make sure `message` is in scope
                }),
            }
        });

        // ################### TIME LOG ###################

        let elapsed = start.elapsed();

        println!(
            "Transaction (Nozomi) submission took: {}",
            format_elapsed(elapsed)
        );

        Ok(response)
    }
}
