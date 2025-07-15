use futures::future::join_all;
use ping::ping;
use reqwest::Client;
use serde_json::json;
use std::{
    net::ToSocketAddrs,
    time::{Duration, Instant},
};

use crate::{
    HEALTH_CHECK_SEC, PING_DURATION_SEC, ZSLOT_REGIONS, ZSlotEndpoint, ZSlotRegionsType, ping_all,
    ping_one,
};

#[derive(Debug)]
pub struct ZeroSlot {
    pub client: Client,
    pub endpoint: ZSlotEndpoint,
    pub auth_key: String,
}

impl ZeroSlot {
    pub async fn new_with_region(region: ZSlotRegionsType, auth_key: String) -> Self {
        let endpoint = ZSLOT_REGIONS
            .iter()
            .find(|r| r.relayer == region)
            .expect("Region not found")
            .clone();

        // Await the ping
        if let Err(err) = ping_one(
            endpoint.relayer_name,
            endpoint.ping_endpoint,
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
        let regions: Vec<(&str, &str)> = ZSLOT_REGIONS
            .iter()
            .map(|r| (r.relayer_name, r.ping_endpoint))
            .collect();

        // Step 1: Ping all regions
        let fastest_index = ping_all(regions.clone(), PING_DURATION_SEC).await;

        // Step 2: Use fastest or fallback
        let endpoint = fastest_index
            .map(|i| ZSLOT_REGIONS[i].clone())
            .unwrap_or_else(|| {
                println!("All region pings failed, falling back to first region.");
                ZSLOT_REGIONS[0].clone()
            });

        println!("Connecting with {} ...", endpoint.relayer_name);

        // Optional: Ping chosen one again
        if let Err(err) = ping_one(&endpoint.relayer_name, &endpoint.ping_endpoint, 2).await {
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

    pub async fn submit_transaction(
        self,
        encoded_tx: &str,
        front_running_protection: bool,
    ) -> anyhow::Result<serde_json::Value> {
        let start = Instant::now();

        let client = Client::new();
        let url = format!("{}{}", self.endpoint.submit_endpoint, self.auth_key);

        let payload = json!({
            "transaction": {
                "content": encoded_tx,
            },
            "frontRunningProtection": front_running_protection,
        });

        let response = client
            .post(url)
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
