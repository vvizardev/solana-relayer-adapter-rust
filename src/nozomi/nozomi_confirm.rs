use futures::future::join_all;
use ping::ping;
use reqwest::Client;
use serde_json::json;
use std::{
    net::ToSocketAddrs,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub enum Nozomi {
    PittDirect,
    TyoDirect,
    SgDirect,
    EwrDirect,
    AmsDirect,
    FraDirect,
    AmsSecure,
    TyoSecure,
    SgSecure,
    EwrSecure,
    PittSecure,
    FraSecure,
}

impl Nozomi {
    fn endpoint(&self) -> &'static str {
        match self {
            Nozomi::PittDirect => "http://pit1.nozomi.temporal.xyz/?c=",
            Nozomi::TyoDirect => "http://tyo1.nozomi.temporal.xyz/?c=",
            Nozomi::SgDirect => "http://sgp1.nozomi.temporal.xyz/?c=",
            Nozomi::EwrDirect => "http://ewr1.nozomi.temporal.xyz/?c=",
            Nozomi::AmsDirect => "http://ams1.nozomi.temporal.xyz/?c=",
            Nozomi::FraDirect => "http://fra2.nozomi.temporal.xyz/?c=",
            Nozomi::AmsSecure => "https://ams1.secure.nozomi.temporal.xyz/?c=",
            Nozomi::TyoSecure => "http://tyo1.secure.nozomi.temporal.xyz/?c=",
            Nozomi::SgSecure => "http://sgp1.secure.nozomi.temporal.xyz/?c=",
            Nozomi::EwrSecure => "https://ewr1.secure.nozomi.temporal.xyz/?c=",
            Nozomi::PittSecure => "https://pit1.secure.nozomi.temporal.xyz/?c=",
            Nozomi::FraSecure => "http://fra2.secure.nozomi.temporal.xyz/?c=",
        }
    }

    pub fn ping_endpoints() -> Vec<(&'static str, &'static str)> {
        vec![
            ("PittDirect", "pit1.nozomi.temporal.xyz"),
            ("TyoDirect", "tyo1.nozomi.temporal.xyz"),
            ("SgDirect", "sgp1.nozomi.temporal.xyz"),
            ("EwrDirect", "ewr1.nozomi.temporal.xyz"),
            ("AmsDirect", "ams1.nozomi.temporal.xyz"),
            ("FraDirect", "fra2.nozomi.temporal.xyz"),
            ("AmsSecure", "ams1.secure.nozomi.temporal.xyz"),
            ("TyoSecure", "tyo1.secure.nozomi.temporal.xyz"),
            ("SgSecure", "sgp1.secure.nozomi.temporal.xyz"),
            ("EwrSecure", "ewr1.secure.nozomi.temporal.xyz"),
            ("PittSecure", "pit1.secure.nozomi.temporal.xyz"),
            ("FraSecure", "fra2.secure.nozomi.temporal.xyz"),
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
                        "{:<12} {:<17} {:>8.3} ms",
                        name,
                        format!("({})", ip),
                        elapsed.as_secs_f64() * 1000.0
                    );
                }
                Err(err) => {
                    println!(
                        "{:<12} {:<17} {}",
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
        region: &Nozomi,
        auth_key: &str,
    ) -> anyhow::Result<serde_json::Value> {
        let start = Instant::now();

        let client = Client::new();
        let url = format!("{}{}", region.endpoint(), auth_key);

        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "sendTransaction",
            "params": [encoded_tx, {"encoding": "base64"}]
        });

        let response = client.post(url).json(&payload).send().await?;

        let data: serde_json::Value = response.json().await?;

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

        Ok(data)
    }
}
