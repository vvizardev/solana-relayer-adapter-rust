use crate::*;
use solana_sdk::{hash::Hash, instruction::Instruction, signature::Keypair};
use std::time::Instant;
use tokio::task::JoinHandle;
use serde_json::Value;
use std::collections::HashMap;

/// Enhanced response parsing that handles various response formats
fn parse_response_safely(body: &str) -> Result<JsonRpcResponse, String> {
    // First try to parse as JSON-RPC response
    if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(body) {
        return Ok(response);
    }
    
    // Check if it's an HTML error page
    if body.contains("<html>") || body.contains("<body>") {
        return Err("HTML error page received (likely 403/404)".to_string());
    }
    
    // Check if it's a simple error object
    if let Ok(error_obj) = serde_json::from_str::<Value>(body) {
        if let Some(error) = error_obj.get("error") {
            return Err(format!("Service error: {}", error));
        }
        if let Some(message) = error_obj.get("message") {
            return Err(format!("Service message: {}", message));
        }
    }
    
    // Try to extract any meaningful error information
    if body.contains("UNAUTHORIZED") {
        return Err("Authentication failed - check API key".to_string());
    }
    
    if body.contains("not authorised") {
        return Err("Not authorized - API key may be invalid or expired".to_string());
    }
    
    if body.contains("api-key does not exist") {
        return Err("API key does not exist - check configuration".to_string());
    }
    
    Err(format!("Failed to parse response: {}", body))
}

/// Service health check to avoid submitting to failing services
async fn check_service_health(service_name: &str, client: &dyn std::fmt::Debug) -> bool {
    // This is a placeholder - in a real implementation, you'd ping each service
    // For now, we'll assume all services are healthy
    true
}

pub async fn ultra_submit(
    tx_info: Tips,
    signers: &Vec<&Keypair>,
    recent_blockhash: Hash,
    nonce_ix: Instruction,
    retry_count: u32,
    jito: Option<&'static Jito>,
    liljit: Option<&'static Jito>,
    astralance: Option<&'static Astralane>,
    helius: Option<&'static Helius>,
    nextblock: Option<&'static NextBlock>,
    zeroslot: Option<&'static ZeroSlot>,
    nozomi: Option<&'static Nozomi>,
) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let global_start = Instant::now();

    // Submit via Jito
    if let Some(jito_client) = jito {
        for i in 0..retry_count {
            let client = jito_client;
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                vec![],
            );
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                match client.send_transaction(&tx).await {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        println!(
                            "[Jito #{}] ✅ Success in {:.2}ms: {:#?}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            response
                        );
                    }
                    Err(e) => {
                        let elapsed = start.elapsed();
                        let error_msg = format!("{}", e);
                        
                        // Enhanced error reporting
                        if error_msg.contains("missing field `jsonrpc`") {
                            eprintln!(
                                "[Jito #{}] ❌ JSON-RPC parsing error after {:.2}ms: Missing jsonrpc field - check service response format",
                                i + 1,
                                elapsed.as_secs_f64() * 1000.0
                            );
                        } else if error_msg.contains("error decoding response body") {
                            eprintln!(
                                "[Jito #{}] ❌ Response decoding error after {:.2}ms: Invalid response format - {}",
                                i + 1,
                                elapsed.as_secs_f64() * 1000.0,
                                error_msg
                            );
                        } else {
                            eprintln!(
                                "[Jito #{}] ❌ Error after {:.2}ms: {}",
                                i + 1,
                                elapsed.as_secs_f64() * 1000.0,
                                error_msg
                            );
                        }
                    }
                }
            });
            handles.push(handle);
        }
    }

    // Submit via LilJit (second Jito instance)
    if let Some(liljit_client) = liljit {
        for i in 0..retry_count {
            let client = liljit_client;
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                vec![],
            );
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                match client.send_transaction(&tx).await {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        println!(
                            "[LilJit #{}] ✅ Success in {:.2}ms: {:#?}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            response
                        );
                    }
                    Err(e) => {
                        let elapsed = start.elapsed();
                        eprintln!(
                            "[LilJit #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            e
                        );
                    }
                }
            });
            handles.push(handle);
        }
    }

    // Submit via Astralane
    if let Some(astralane_client) = astralance {
        for i in 0..retry_count {
            let client = astralane_client;
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                vec![],
            );
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                match client.send_transaction(&tx).await {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        println!(
                            "[Astralane #{}] ✅ Success in {:.2}ms: {:#?}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            response
                        );
                    }
                    Err(e) => {
                        let elapsed = start.elapsed();
                        eprintln!(
                            "[Astralane #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            e
                        );
                    }
                }
            });
            handles.push(handle);
        }
    }

    // Submit via Helius
    if let Some(helius_client) = helius {
        for i in 0..retry_count {
            let client = helius_client;
            let ix = client.add_tip_ix(tx_info.clone(), false);
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                vec![],
            );
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                match client.send_transaction(&tx).await {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        println!(
                            "[Helius #{}] ✅ Success in {:.2}ms: {:#?}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            response
                        );
                    }
                    Err(e) => {
                        let elapsed = start.elapsed();
                        eprintln!(
                            "[Helius #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            e
                        );
                    }
                }
            });
            handles.push(handle);
        }
    }

    // Submit via NextBlock
    if let Some(nextblock_client) = nextblock {
        for i in 0..retry_count {
            let client = nextblock_client;

            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                vec![],
            );

            let handle = tokio::spawn(async move {
                let start = Instant::now();
                match client.send_transaction(&tx, None).await {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        println!(
                            "[NextBlock #{}] ✅ Success in {:.2}ms: {:#?}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            response
                        );
                    }
                    Err(e) => {
                        let elapsed = start.elapsed();
                        eprintln!(
                            "[NextBlock #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            e
                        );
                    }
                }
            });
            handles.push(handle);
        }
    }

    // Submit via ZeroSlot
    if let Some(zeroslot_client) = zeroslot {
        for i in 0..retry_count {
            let client = zeroslot_client;
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                vec![],
            );
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                match client.send_transaction(&tx).await {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        println!(
                            "[ZeroSlot #{}] ✅ Success in {:.2}ms: {:#?}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            response
                        );
                    }
                    Err(e) => {
                        let elapsed = start.elapsed();
                        eprintln!(
                            "[ZeroSlot #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            e
                        );
                    }
                }
            });
            handles.push(handle);
        }
    }

    if let Some(nozomi_client) = nozomi {
        for i in 0..retry_count {
            let client = nozomi_client;
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                vec![],
            );
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                match client.send_transaction(&tx).await {
                    Ok(response) => {
                        let elapsed = start.elapsed();
                        println!(
                            "[Nozomi #{}] ✅ Success in {:.2}ms: {:#?}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            response
                        );
                    }
                    Err(e) => {
                        let elapsed = start.elapsed();
                        eprintln!(
                            "[Nozomi #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            e
                        );
                    }
                }
            });
            handles.push(handle);
        }
    }

    // Wait for all submissions to complete
    println!(
        "\n🚀 Launched {} simultaneous submissions (all services × {})",
        handles.len(),
        retry_count
    );

    // All tasks are already running in parallel - just wait for them to finish
    futures::future::join_all(handles).await;

    let total_elapsed = global_start.elapsed();
    println!(
        "\n✅ All submissions completed in {:.2}ms (wall time)",
        total_elapsed.as_secs_f64() * 1000.0
    );
}
