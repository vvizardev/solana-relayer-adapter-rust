use futures::future::join_all;
use ping::ping;
use reqwest::Client;
use serde_json::json;
use std::{
    net::ToSocketAddrs,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub enum ZeroSlot {
    Frankfurt,
    NewYork,
    AMS,
    LA,
    Tokyo,
}

impl ZeroSlot {
    fn endpoint(&self) -> &'static str {
        match self {
            ZeroSlot::NewYork => "https://ny.0slot.trade?api-key=",
            ZeroSlot::Frankfurt => "https://de.0slot.trade?api-key=",
            ZeroSlot::AMS => "https://ams.0slot.trade?api-key=",
            ZeroSlot::LA => "https://la.0slot.trade?api-key=",
            ZeroSlot::Tokyo => "https://jp.0slot.trade?api-key=",
        }
    }

    pub fn ping_endpoints() -> Vec<(&'static str, &'static str)> {
        vec![
            ("ZeroSlot-NewYork", "ny.0slot.trade"),
            ("ZeroSlot-Frankfurt", "de.0slot.trade"),
            ("ZeroSlot-AMS", "ams.0slot.trade"),
            ("ZeroSlot-LA", "la.0slot.trade"),
            ("ZeroSlot-Tokyo", "jp.0slot.trade"),
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
        region: ZeroSlot,
        auth_header: &str,
        front_running_protection: bool,
    ) -> anyhow::Result<serde_json::Value> {
        let start = Instant::now();

        let client = Client::new();
        let endpoint = region.endpoint();

        let payload = json!({
            "transaction": {
                "content": encoded_tx,
            },
            "frontRunningProtection": front_running_protection,
        });

        let response = client
            .post(endpoint)
            .header("Authorization", auth_header)
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
