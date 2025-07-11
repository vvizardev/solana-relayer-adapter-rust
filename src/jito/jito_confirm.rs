use futures::future::join_all;
use ping::ping;
use reqwest::Client;
use serde_json::json;
use std::{
    net::ToSocketAddrs,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub enum Jito {
    Mainnet,
    Amsterdam,
    Frankfurt,
    London,
    NY,
    SLC,
    SG,
    Tokyo,
}

impl Jito {
    fn endpoint(&self) -> &'static str {
        match self {
            Jito::Mainnet => "https://mainnet.block-engine.jito.wtf/api/v1/transactions",
            Jito::Amsterdam => {
                "https://amsterdam.mainnet.block-engine.jito.wtf/api/v1/transactions"
            }
            Jito::Frankfurt => {
                "https://frankfurt.mainnet.block-engine.jito.wtf/api/v1/transactions"
            }
            Jito::London => "https://london.mainnet.block-engine.jito.wtf/api/v1/transactions",
            Jito::NY => "https://ny.mainnet.block-engine.jito.wtf/api/v1/transactions",
            Jito::SLC => "https://slc.mainnet.block-engine.jito.wtf/api/v1/transactions",
            Jito::SG => "https://singapore.mainnet.block-engine.jito.wtf/api/v1/transactions",
            Jito::Tokyo => "https://tokyo.mainnet.block-engine.jito.wtf/api/v1/transactions",
        }
    }

    pub fn ping_endpoints() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Jito-Mainnet", "mainnet.block-engine.jito.wtf"),
            ("Jito-Amsterdam", "amsterdam.mainnet.block-engine.jito.wtf"),
            ("Jito-Frankfurt", "frankfurt.mainnet.block-engine.jito.wtf"),
            ("Jito-London", "london.mainnet.block-engine.jito.wtf"),
            ("Jito-NY", "ny.mainnet.block-engine.jito.wtf"),
            ("Jito-SLC", "slc.mainnet.block-engine.jito.wtf"),
            ("Jito-SG", "singapore.mainnet.block-engine.jito.wtf"),
            ("Jito-Tokyo", "tokyo.mainnet.block-engine.jito.wtf"),
        ]
    }

    pub async fn icmp_ping_all(regions: Vec<(&'static str, &'static str)>) {
        let timeout = Duration::from_secs(2);
        let ident = 0xABCD;

        let futures = regions.into_iter().map(|(name, host)| async move {
            // Resolve hostname to IP
            let ip = match (host, 0)
                .to_socket_addrs()
                .ok()
                .and_then(|mut iter| iter.next())
            {
                Some(addr) => addr.ip(),
                None => {
                    println!(
                        "{:<12} {:<17} {}",
                        name, "N/A", "Failed to resolve hostname"
                    );
                    return;
                }
            };

            // Measure RTT
            let start = Instant::now();
            let result = ping(
                ip,
                Some(timeout),
                Some(64),
                Some(ident),
                Some(1),
                Some(&[0; 24]),
            );
            let elapsed = start.elapsed();

            match result {
                Ok(_) => {
                    println!(
                        "{:<22} {:<17} {:>8.3} ms",
                        name,
                        format!("({})", ip),
                        elapsed.as_secs_f64() * 1000.0
                    );
                }
                Err(err) => {
                    println!(
                        "{:<22} {:<17} {}",
                        name,
                        format!("({})", ip),
                        format!("Ping failed: {}", err)
                    );
                }
            }
        });

        join_all(futures).await;
    }

    pub async fn submit_transaction(
        encoded_tx: &str,
        region: &Jito,
    ) -> anyhow::Result<serde_json::Value> {
        let start = Instant::now();

        let client = Client::new();
        let rpc_endpoint = region.endpoint();

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [encoded_tx , {
                "encoding": "base64"
              }],

        });

        let response = client
            .post(&format!("{}?bundleOnly=false", rpc_endpoint))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        // ################### TIME LOG ###################

        let elapsed = start.elapsed();
        let secs = elapsed.as_secs();
        let nanos = elapsed.subsec_nanos();

        let seconds = secs;
        let millis = nanos / 1_000_000;
        let micros = (nanos % 1_000_000) / 1_000;

        let mut parts = vec![];

        if seconds > 0 {
            parts.push(format!("{}s", seconds));
        }
        if millis > 0 {
            parts.push(format!("{}ms", millis));
        }
        if micros > 0 && millis == 0 {
            // Only show µs if ms == 0 to avoid redundancy
            parts.push(format!("{}µs", micros));
        }

        if parts.is_empty() {
            parts.push("0µs".to_string()); // fallback if literally nothing
        }

        println!("Transaction submission took: {}", parts.join(" : "));

        Ok(json)
    }
}
