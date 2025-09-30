use reqwest::Client;
use serde_json::json;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction, hash::Hash, instruction::Instruction, message::AddressLookupTableAccount, native_token::sol_to_lamports, pubkey::Pubkey, signature::Keypair, system_instruction
};
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::{
    HEALTH_CHECK_SEC, JITO_MIN_TIP, JITO_REGIONS, JITO_TIP, JitoEndpoint, JitoRegionsType,
    JsonRpcResponse, PING_DURATION_SEC, Tips, TransactionBuilder, build_v0_bs64, format_elapsed,
    ping_all, ping_one, simulate,
};

#[derive(Debug)]
pub struct Jito {
    pub client: Client,
    pub endpoint: JitoEndpoint,
    pub auth_key: Option<String>,
}

impl TransactionBuilder for Jito {
    fn build_v0_bs64(
        &self,
        ixs: Vec<Instruction>,
        fee_payer: &Pubkey,
        signers: &Vec<&Keypair>,
        recent_blockhash: Hash,
        nonce_ix: Option<Instruction>,
        alt: Vec<AddressLookupTableAccount>,
    ) -> String {
        build_v0_bs64(ixs, fee_payer, signers, recent_blockhash, nonce_ix, alt)
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

impl Jito {
    pub async fn new_with_region(region: JitoRegionsType, auth_key: Option<String>) -> Self {
        let endpoint = JITO_REGIONS
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

    pub async fn new_auto(auth_key: Option<String>) -> Self {
        let regions: Vec<(String, String)> = JITO_REGIONS
            .iter()
            .map(|r| (r.relayer_name.to_string(), r.ping_endpoint.to_string()))
            .collect();

        // Step 1: Ping all regions
        let fastest_index = ping_all(regions.clone(), PING_DURATION_SEC).await;

        // Step 2: Use fastest or fallback
        let endpoint = fastest_index
            .map(|i| JITO_REGIONS[i].clone())
            .unwrap_or_else(|| {
                println!("All region pings failed, falling back to first region.");
                JITO_REGIONS[0].clone()
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
        // let client = self.client.clone();
        // let endpoint = self.endpoint.clone();
        // let relayer_name = endpoint.relayer_name.clone();
        // let rpc_url = format!("https://{}", endpoint.ping_endpoint.clone());

        // tokio::spawn(async move {
        //     let payload = json!({
        //         "jsonrpc": "2.0",
        //         "id": 1,
        //         "method": "getHealth"
        //     });

        //     loop {
        //         match client.post(&rpc_url).json(&payload).send().await {
        //             Ok(response) if response.status().is_success() => {
        //                 println!("{} health check successful", relayer_name);
        //             }
        //             Ok(response) => {
        //                 eprintln!(
        //                     "{} health check failed with status: {}",
        //                     relayer_name,
        //                     response.status()
        //                 );
        //             }
        //             Err(err) => {
        //                 eprintln!("{} health check request error: {:?}", relayer_name, err);
        //             }
        //         }

        //         sleep(Duration::from_secs(interval_sec)).await;
        //     }
        // });
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

        let relayer_fee = tip_config.tip_sol_amount.max(JITO_MIN_TIP); // use `.max()` for clarity

        let recipient = Pubkey::from_str_const(JITO_TIP[tip_config.tip_addr_idx as usize]);
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

        let url = if let Some(auth_key) = &self.auth_key {
            format!(
                "{}/api/v1/transactions?uuid={}",
                self.endpoint.submit_endpoint, auth_key
            )
        } else {
            format!("{}/api/v1/transactions", self.endpoint.submit_endpoint)
        };

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [encoded_tx, {"encoding": "base64"}]
        });

        let response = self.client.post(url).json(&payload).send().await?;

        let body = response.text().await?;
        println!("Raw response body:\n{:#?}", body);

        // Parse and return response body as JSON
        let response: JsonRpcResponse = serde_json::from_str(&body)?;

        // ################### TIME LOG ###################

        let elapsed = start.elapsed();

        println!(
            "Transaction (Jito) submission took: {}",
            format_elapsed(elapsed)
        );

        Ok(response)
    }

    pub async fn send_bundle(&self, encoded_txs: &[String]) -> anyhow::Result<JsonRpcResponse> {
        let start = Instant::now();

        let url = if let Some(auth_key) = &self.auth_key {
            format!(
                "{}/api/v1/bundles?uuid={}",
                self.endpoint.submit_endpoint, auth_key
            )
        } else {
            format!("{}/api/v1/bundles", self.endpoint.submit_endpoint)
        };

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendBundle",
            "params": [encoded_txs, {"encoding": "base64"}]
        });

        let response = self.client.post(url).json(&payload).send().await?;
        let body = response.text().await?;
        println!("Raw response body:\n{}", body);

        // Parse and return response body as JSON
        let response: JsonRpcResponse = serde_json::from_str(&body)?;

        // ################### TIME LOG ###################

        let elapsed = start.elapsed();

        println!("Transaction submission took: {}", format_elapsed(elapsed));

        Ok(response)
    }
}
