use crate::*;
use once_cell::sync::Lazy;
use serde_json::Value;
use solana_sdk::{hash::Hash, instruction::Instruction, signature::Keypair};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use std::time::{Duration, Instant as StdInstant};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

/// Service health tracking for circuit breaker pattern
#[derive(Debug, Clone)]
struct ServiceHealth {
    consecutive_failures: u32,
    last_failure: Option<StdInstant>,
    is_circuit_open: bool,
    circuit_open_time: Option<StdInstant>,
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self {
            consecutive_failures: 0,
            last_failure: None,
            is_circuit_open: false,
            circuit_open_time: None,
        }
    }
}

impl ServiceHealth {
    fn record_success(&mut self) {
        self.consecutive_failures = 0;
        self.is_circuit_open = false;
        self.circuit_open_time = None;
    }

    fn record_failure(&mut self) {
        self.consecutive_failures += 1;
        self.last_failure = Some(StdInstant::now());

        // Open circuit after 3 consecutive failures
        if self.consecutive_failures >= 3 {
            self.is_circuit_open = true;
            self.circuit_open_time = Some(StdInstant::now());
        }
    }

    fn should_attempt_request(&mut self) -> bool {
        // If circuit is closed, allow requests
        if !self.is_circuit_open {
            return true;
        }

        // If circuit is open, check if enough time has passed to try again
        if let Some(open_time) = self.circuit_open_time {
            let elapsed = open_time.elapsed();
            // Try again after 30 seconds
            if elapsed >= Duration::from_secs(30) {
                self.is_circuit_open = false;
                self.circuit_open_time = None;
                return true;
            }
        }

        false
    }
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

                        // Enhanced error reporting with detailed analysis
                        eprintln!(
                            "[Jito #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            error_msg
                        );

                        // Provide specific guidance based on error type
                        if error_msg.contains("missing field `jsonrpc`") {
                            eprintln!(
                                "   💡 This service returned a response without the required 'jsonrpc' field"
                            );
                        } else if error_msg.contains("error decoding response body") {
                            eprintln!(
                                "   💡 The service returned malformed JSON - check service status"
                            );
                        } else if error_msg.contains("expected value") {
                            eprintln!("   💡 Invalid JSON structure received from service");
                        } else if error_msg.contains("UNAUTHORIZED") {
                            eprintln!("   💡 Authentication failed - verify API key is valid");
                        } else if error_msg.contains("not authorised") {
                            eprintln!("   💡 API key may be invalid or expired");
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
                        let error_msg = format!("{}", e);

                        // Enhanced error reporting with detailed analysis
                        eprintln!(
                            "[LilJit #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            error_msg
                        );

                        // Provide specific guidance based on error type
                        if error_msg.contains("missing field `jsonrpc`") {
                            eprintln!(
                                "   💡 This service returned a response without the required 'jsonrpc' field"
                            );
                        } else if error_msg.contains("error decoding response body") {
                            eprintln!(
                                "   💡 The service returned malformed JSON - check service status"
                            );
                        } else if error_msg.contains("expected value") {
                            eprintln!("   💡 Invalid JSON structure received from service");
                        } else if error_msg.contains("UNAUTHORIZED") {
                            eprintln!("   💡 Authentication failed - verify API key is valid");
                        } else if error_msg.contains("not authorised") {
                            eprintln!("   💡 API key may be invalid or expired");
                        }
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
                        let error_msg = format!("{}", e);

                        // Enhanced error reporting with detailed analysis
                        eprintln!(
                            "[Astralane #{}] ❌ Error after {:.2}ms: {}",
                            i + 1,
                            elapsed.as_secs_f64() * 1000.0,
                            error_msg
                        );

                        // Provide specific guidance based on error type
                        if error_msg.contains("missing field `jsonrpc`") {
                            eprintln!(
                                "   💡 This service returned a response without the required 'jsonrpc' field"
                            );
                        } else if error_msg.contains("error decoding response body") {
                            eprintln!(
                                "   💡 The service returned malformed JSON - check service status"
                            );
                        } else if error_msg.contains("expected value") {
                            eprintln!("   💡 Invalid JSON structure received from service");
                        } else if error_msg.contains("UNAUTHORIZED") {
                            eprintln!("   💡 Authentication failed - verify API key is valid");
                        } else if error_msg.contains("not authorised") {
                            eprintln!("   💡 API key may be invalid or expired");
                        }
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
