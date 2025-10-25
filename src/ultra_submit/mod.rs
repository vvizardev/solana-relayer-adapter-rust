use crate::*;
use solana_sdk::message::AddressLookupTableAccount;
use solana_sdk::{hash::Hash, instruction::Instruction, signature::Keypair};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use futures::future::join_all;

/// Submission result tracking
#[derive(Debug, Clone)]
pub struct SubmissionResult {
    pub service_name: String,
    pub attempt: u32,
    pub success: bool,
    pub latency_ms: f64,
    pub error: Option<String>,
}

/// Service configuration for submission
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub name: &'static str,
    pub client: ServiceClient,
}

/// Enum for different service clients
#[derive(Debug, Clone)]
pub enum ServiceClient {
    Jito(&'static Jito),
    LilJit(&'static Jito),
    Astralane(&'static Astralane),
    Helius(&'static Helius),
    NextBlock(&'static NextBlock),
    ZeroSlot(&'static ZeroSlot),
    Nozomi(&'static Nozomi),
    BlockRazor(&'static BlockRazor),
    BloxRoute(&'static BloxRoute),
}

/// Generic submission function to eliminate code duplication
async fn submit_to_service(
    config: ServiceConfig,
    tx_info: Tips,
    signers: &'static Vec<&'static Keypair>,
    recent_blockhash: Hash,
    nonce_ix: Instruction,
    alt: Vec<AddressLookupTableAccount>,
    attempt: u32,
) -> SubmissionResult {
    let start = Instant::now();
    let service_name = config.name.to_string();
    
    let result: anyhow::Result<serde_json::Value> = match config.client {
        ServiceClient::Jito(client) => {
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx).await.map(|resp| serde_json::to_value(resp).unwrap())
        }
        ServiceClient::LilJit(client) => {
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx).await.map(|resp| serde_json::to_value(resp).unwrap())
        }
        ServiceClient::Astralane(client) => {
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx).await
        }
        ServiceClient::Helius(client) => {
            let ix = client.add_tip_ix(tx_info.clone(), false);
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx).await.map(|resp| serde_json::to_value(resp).unwrap())
        }
        ServiceClient::NextBlock(client) => {
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx, None).await.map(|resp| serde_json::to_value(resp).unwrap())
        }
        ServiceClient::ZeroSlot(client) => {
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx).await.map(|resp| serde_json::to_value(resp).unwrap())
        }
        ServiceClient::Nozomi(client) => {
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx).await.map(|resp| serde_json::to_value(resp).unwrap())
        }
        ServiceClient::BlockRazor(client) => {
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx).await.map(|resp| serde_json::to_value(resp).unwrap())
        }
        ServiceClient::BloxRoute(client) => {
            let ix = client.add_tip_ix(tx_info.clone());
            let tx = client.build_v0_bs64(
                ix,
                &tx_info.payer,
                signers,
                recent_blockhash,
                Some(nonce_ix.clone()),
                alt.clone()
            );
            client.send_transaction(&tx).await.map(|resp| serde_json::to_value(resp).unwrap())
        }
    };

    let latency_ms = start.elapsed().as_secs_f64() * 1000.0;

    match result {
        Ok(response) => {
            println!(
                "[{} #{}] ✅ Success in {:.2}ms: {:#?}",
                service_name, attempt + 1, latency_ms, response
            );
            SubmissionResult {
                service_name,
                attempt,
                success: true,
                latency_ms,
                error: None,
            }
        }
        Err(e) => {
            let error_msg = format!("{}", e);
            
            // Enhanced error reporting with detailed analysis
            eprintln!(
                "[{} #{}] ❌ Error after {:.2}ms: {}",
                service_name, attempt + 1, latency_ms, error_msg
            );

            // Provide specific guidance based on error type
            if error_msg.contains("missing field `jsonrpc`") {
                eprintln!("   💡 This service returned a response without the required 'jsonrpc' field");
            } else if error_msg.contains("error decoding response body") {
                eprintln!("   💡 The service returned malformed JSON - check service status");
            } else if error_msg.contains("expected value") {
                eprintln!("   💡 Invalid JSON structure received from service");
            } else if error_msg.contains("UNAUTHORIZED") {
                eprintln!("   💡 Authentication failed - verify API key is valid");
            } else if error_msg.contains("not authorised") {
                eprintln!("   💡 API key may be invalid or expired");
            }

            SubmissionResult {
                service_name,
                attempt,
                success: false,
                latency_ms,
                error: Some(error_msg),
            }
        }
    }
}

pub async fn ultra_submit(
    tx_info: Tips,
    signers: &'static Vec<&'static Keypair>,
    recent_blockhash: Hash,
    nonce_ix: Instruction,
    alt: Vec<AddressLookupTableAccount>,
    retry_count: u32,
    jito: Option<&'static Jito>,
    liljit: Option<&'static Jito>,
    astralance: Option<&'static Astralane>,
    helius: Option<&'static Helius>,
    nextblock: Option<&'static NextBlock>,
    zeroslot: Option<&'static ZeroSlot>,
    nozomi: Option<&'static Nozomi>,
    blockrazor: Option<&'static BlockRazor>,
    bloxroute: Option<&'static BloxRoute>,
) {
    let global_start = Instant::now();
    println!("🚀 Starting ultra_submit process with optimized multi-threading...");

    // Collect all available services
    let mut services = Vec::new();
    
    if let Some(client) = jito {
        services.push(ServiceConfig {
            name: "Jito",
            client: ServiceClient::Jito(client),
        });
    }
    
    if let Some(client) = liljit {
        services.push(ServiceConfig {
            name: "LilJit",
            client: ServiceClient::LilJit(client),
        });
    }
    
    if let Some(client) = astralance {
        services.push(ServiceConfig {
            name: "Astralane",
            client: ServiceClient::Astralane(client),
        });
    }
    
    if let Some(client) = helius {
        services.push(ServiceConfig {
            name: "Helius",
            client: ServiceClient::Helius(client),
        });
    }
    
    if let Some(client) = nextblock {
        services.push(ServiceConfig {
            name: "NextBlock",
            client: ServiceClient::NextBlock(client),
        });
    }
    
    if let Some(client) = zeroslot {
        services.push(ServiceConfig {
            name: "ZeroSlot",
            client: ServiceClient::ZeroSlot(client),
        });
    }
    
    if let Some(client) = nozomi {
        services.push(ServiceConfig {
            name: "Nozomi",
            client: ServiceClient::Nozomi(client),
        });
    }
    
    if let Some(client) = blockrazor {
        services.push(ServiceConfig {
            name: "BlockRazor",
            client: ServiceClient::BlockRazor(client),
        });
    }
    
    if let Some(client) = bloxroute {
        services.push(ServiceConfig {
            name: "BloxRoute",
            client: ServiceClient::BloxRoute(client),
        });
    }

    let total_services = services.len();
    let total_submissions = total_services * retry_count as usize;
    
    println!("📊 Configuration: {} services × {} retries = {} total submissions", 
             total_services, retry_count, total_submissions);

    // Prepare all submission tasks
    let preparation_start = Instant::now();
    let mut handles = Vec::new();

    // Create all submission tasks using the generic function
    // Use Arc to share data efficiently across tasks
    let tx_info_arc = Arc::new(tx_info);
    let nonce_ix_arc = Arc::new(nonce_ix);
    let alt_arc = Arc::new(alt);

    for service in services {
        for attempt in 0..retry_count {
            let service_config = service.clone();
            let tx_info_shared = Arc::clone(&tx_info_arc);
            let nonce_ix_shared = Arc::clone(&nonce_ix_arc);
            let alt_shared = Arc::clone(&alt_arc);

            let handle = tokio::spawn(async move {
                submit_to_service(
                    service_config,
                    (*tx_info_shared).clone(),
                    signers,
                    recent_blockhash,
                    (*nonce_ix_shared).clone(),
                    (*alt_shared).clone(),
                    attempt,
                ).await
            });
            handles.push(handle);
        }
    }

    let preparation_elapsed = preparation_start.elapsed();
    println!(
        "⚡ Preparation phase completed in {:.2}ms",
        preparation_elapsed.as_secs_f64() * 1000.0
    );

    // Execute all submissions simultaneously with no delays
    let execution_start = Instant::now();
    println!(
        "\n🚀 Launching {} simultaneous submissions across {} services",
        total_submissions, total_services
    );

    // Execute all tasks truly simultaneously - maximum parallelism
    let results = join_all(handles).await;

    let execution_elapsed = execution_start.elapsed();
    let total_elapsed = global_start.elapsed();

    // Collect and analyze results
    let mut success_count = 0;
    let mut total_latency = 0.0;
    let mut service_stats: HashMap<String, (u32, u32, f64)> = HashMap::new(); // (success, total, total_latency)

    for result in results {
        if let Ok(submission_result) = result {
            if submission_result.success {
                success_count += 1;
            }
            total_latency += submission_result.latency_ms;
            
            let stats = service_stats.entry(submission_result.service_name.clone()).or_insert((0, 0, 0.0));
            stats.1 += 1; // total attempts
            stats.2 += submission_result.latency_ms; // total latency
            if submission_result.success {
                stats.0 += 1; // successful attempts
            }
        }
    }

    // Print detailed results
    println!(
        "\n✅ All submissions completed in {:.2}ms (wall time)",
        total_elapsed.as_secs_f64() * 1000.0
    );

    println!("\n📊 Performance Summary:");
    println!("   • Preparation: {:.2}ms", preparation_elapsed.as_secs_f64() * 1000.0);
    println!("   • Execution: {:.2}ms", execution_elapsed.as_secs_f64() * 1000.0);
    println!("   • Total: {:.2}ms", total_elapsed.as_secs_f64() * 1000.0);
    println!("   • Success Rate: {}/{} ({:.1}%)", 
             success_count, total_submissions, 
             (success_count as f64 / total_submissions as f64) * 100.0);
    
    if success_count > 0 {
        println!("   • Average Latency: {:.2}ms", total_latency / success_count as f64);
    }

    // Service-specific statistics
    println!("\n📈 Service Performance:");
    for (service_name, (success, total, latency)) in service_stats {
        let success_rate = (success as f64 / total as f64) * 100.0;
        let avg_latency = if success > 0 { latency / success as f64 } else { 0.0 };
        println!("   • {}: {}/{} ({:.1}%) | Avg: {:.2}ms", 
                 service_name, success, total, success_rate, avg_latency);
    }
}

/// Convenience function for ultra_submit with simplified parameters
/// This function provides a cleaner interface for common use cases
pub async fn ultra_submit_simple(
    tx_info: Tips,
    signers: &'static Vec<&'static Keypair>,
    recent_blockhash: Hash,
    nonce_ix: Instruction,
    alt: Vec<AddressLookupTableAccount>,
    retry_count: u32,
    services: Vec<ServiceConfig>,
) {
    let global_start = Instant::now();
    println!("🚀 Starting ultra_submit_simple with {} services", services.len());

    let total_services = services.len();
    let total_submissions = total_services * retry_count as usize;
    
    println!("📊 Configuration: {} services × {} retries = {} total submissions", 
             total_services, retry_count, total_submissions);

    // Prepare all submission tasks
    let preparation_start = Instant::now();
    let mut handles = Vec::new();

    // Use Arc to share data efficiently across tasks
    let tx_info_arc = Arc::new(tx_info);
    let nonce_ix_arc = Arc::new(nonce_ix);
    let alt_arc = Arc::new(alt);

    for service in services {
        for attempt in 0..retry_count {
            let service_config = service.clone();
            let tx_info_shared = Arc::clone(&tx_info_arc);
            let nonce_ix_shared = Arc::clone(&nonce_ix_arc);
            let alt_shared = Arc::clone(&alt_arc);

            let handle = tokio::spawn(async move {
                submit_to_service(
                    service_config,
                    (*tx_info_shared).clone(),
                    signers,
                    recent_blockhash,
                    (*nonce_ix_shared).clone(),
                    (*alt_shared).clone(),
                    attempt,
                ).await
            });
            handles.push(handle);
        }
    }

    let preparation_elapsed = preparation_start.elapsed();
    println!(
        "⚡ Preparation phase completed in {:.2}ms",
        preparation_elapsed.as_secs_f64() * 1000.0
    );

    // Execute all submissions simultaneously with no delays
    let execution_start = Instant::now();
    println!(
        "\n🚀 Launching {} simultaneous submissions across {} services",
        total_submissions, total_services
    );

    // Execute all tasks truly simultaneously - maximum parallelism
    let results = join_all(handles).await;

    let execution_elapsed = execution_start.elapsed();
    let total_elapsed = global_start.elapsed();

    // Collect and analyze results
    let mut success_count = 0;
    let mut total_latency = 0.0;
    let mut service_stats: HashMap<String, (u32, u32, f64)> = HashMap::new(); // (success, total, total_latency)

    for result in results {
        if let Ok(submission_result) = result {
            if submission_result.success {
                success_count += 1;
            }
            total_latency += submission_result.latency_ms;
            
            let stats = service_stats.entry(submission_result.service_name.clone()).or_insert((0, 0, 0.0));
            stats.1 += 1; // total attempts
            stats.2 += submission_result.latency_ms; // total latency
            if submission_result.success {
                stats.0 += 1; // successful attempts
            }
        }
    }

    // Print detailed results
    println!(
        "\n✅ All submissions completed in {:.2}ms (wall time)",
        total_elapsed.as_secs_f64() * 1000.0
    );

    println!("\n📊 Performance Summary:");
    println!("   • Preparation: {:.2}ms", preparation_elapsed.as_secs_f64() * 1000.0);
    println!("   • Execution: {:.2}ms", execution_elapsed.as_secs_f64() * 1000.0);
    println!("   • Total: {:.2}ms", total_elapsed.as_secs_f64() * 1000.0);
    println!("   • Success Rate: {}/{} ({:.1}%)", 
             success_count, total_submissions, 
             (success_count as f64 / total_submissions as f64) * 100.0);
    
    if success_count > 0 {
        println!("   • Average Latency: {:.2}ms", total_latency / success_count as f64);
    }

    // Service-specific statistics
    println!("\n📈 Service Performance:");
    for (service_name, (success, total, latency)) in service_stats {
        let success_rate = (success as f64 / total as f64) * 100.0;
        let avg_latency = if success > 0 { latency / success as f64 } else { 0.0 };
        println!("   • {}: {}/{} ({:.1}%) | Avg: {:.2}ms", 
                 service_name, success, total, success_rate, avg_latency);
    }
}
