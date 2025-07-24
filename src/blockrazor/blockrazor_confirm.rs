use reqwest::{
    Client,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde_json::json;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, hash::Hash, instruction::Instruction,
    native_token::sol_to_lamports, pubkey::Pubkey, signature::Keypair, system_instruction,
};
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::{
    build_v0_bs64, format_elapsed, ping_all, ping_one, simulate, BRazorEndpoint, BRazorRegionsType, BlockRazorResponse, HealthResponse, JsonRpcResponse, Tips, TransactionBuilder, BLOCKRAZOR_MIN_TIP, BLOCKRAZOR_TIP, BRAZOR_REGIONS, HEALTH_CHECK_SEC, PING_DURATION_SEC
};

#[derive(Debug)]
pub struct BlockRazor {
    pub client: Client,
    pub endpoint: BRazorEndpoint,
    pub auth_key: String,
}

impl TransactionBuilder for BlockRazor {
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

impl BlockRazor {
    pub async fn new_with_region(region: BRazorRegionsType, auth_key: String) -> Self {
        let endpoint = BRAZOR_REGIONS
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
        let regions: Vec<(String, String)> = BRAZOR_REGIONS
            .iter()
            .map(|r| (r.relayer_name.to_string(), r.ping_endpoint.to_string()))
            .collect();

        // Step 1: Ping all regions
        let fastest_index = ping_all(regions.clone(), PING_DURATION_SEC).await;

        // Step 2: Use fastest or fallback
        let endpoint = fastest_index
            .map(|i| BRAZOR_REGIONS[i].clone())
            .unwrap_or_else(|| {
                println!("All region pings failed, falling back to first region.");
                BRAZOR_REGIONS[0].clone()
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
        let relayer_name = endpoint.relayer_name.clone();
        let auth_key = self.auth_key.clone();
        let url = format!("{}/health", self.endpoint.submit_endpoint);

        tokio::spawn(async move {
            loop {
                let result = async {
                    let response = client
                        .get(&url)
                        .header("Content-Type", "application/json")
                        .header("apikey", &auth_key)
                        .send()
                        .await?;

                    let body = response.text().await?;
                    println!("Raw health response: {}", body);

                    let parsed: Result<HealthResponse, _> = serde_json::from_str(&body);
                    if let Ok(hr) = parsed {
                        println!("Health result [{}]: {}", relayer_name, hr.result);
                    } else {
                        println!(
                            "Failed to parse health response from {}: {}",
                            relayer_name, body
                        );
                    }

                    Ok::<(), anyhow::Error>(())
                }
                .await;

                if let Err(e) = result {
                    eprintln!("Health check error [{}]: {}", relayer_name, e);
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

        let relayer_fee = tip_config.tip_sol_amount.max(BLOCKRAZOR_MIN_TIP); // use `.max()` for clarity

        let recipient = Pubkey::from_str_const(BLOCKRAZOR_TIP[tip_config.tip_addr_idx as usize]);
        let transfer_ix = system_instruction::transfer(
            &tip_config.payer,
            &recipient,
            sol_to_lamports(relayer_fee),
        );
        ixs.push(transfer_ix);

        ixs
    }

    pub async fn send_transaction(&self, encoded_tx: &str) -> anyhow::Result<BlockRazorResponse> {
        let start = Instant::now();

        let client = Client::new();
        let url = format!("{}/sendTransaction", self.endpoint.submit_endpoint);

        let payload = json!({
            "transaction": {
                "content": encoded_tx,
                "mode": "fast"
            },
        });

        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("apikey", &self.auth_key)
            .json(&payload)
            .send()
            .await?;

        let body = response.text().await?;
        println!("Raw response body:\n{}", body);

        // Parse and return response body as JSON
        let response: BlockRazorResponse = serde_json::from_str(&body)?;

        // ################### TIME LOG ###################

        let elapsed = start.elapsed();

        println!(
            "Transaction (BlockLazor) submission took: {}",
            format_elapsed(elapsed)
        );

        Ok(response)
    }
}
