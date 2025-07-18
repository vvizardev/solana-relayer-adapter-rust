# ⚡ Solana Low-Latency Relayer Adapter

A unified client interface for interacting with top Solana transaction relayers, enabling **low-latency**, **high-performance** transaction submission across:

- 🛰️ [Jito](https://docs.jito.wtf/lowlatencytxnsend/)
- 🚅 [Nozomi](https://use.temporal.xyz/nozomi/transaction-submission)
- ⚡ [ZeroSlot](https://0slot.trade/docs.php)
- 🧠 [BloxRoute](https://docs.bloxroute.com/solana/trader-api/best-performance-for-landing-transactions)
- 🧱 [NextBlock](https://docs.nextblock.io/getting-started/quickstart)
- 🪓 [Blockrazor](https://blockrazor.gitbook.io/blockrazor/solana/send-transaction/rust)
- ✨ [Astralane](https://astralane.gitbook.io/docs/low-latency/endpoints-and-configs)

---

## 📦 Features

- Easy API client setup using `OnceCell`
- Automatic region-based endpoint selection
- Health check and ping latency diagnostics
- Built-in support for `.env` API key loading
- Unified interface for sending Solana transactions

---

## 🚀 Usage

### 1. Add to Your Project

- Cargo CLI
```bash
cargo add solana-relayer-adapter-rust
```

- Cargo TOML
```toml
# Cargo.toml
[dependencies]
...
solana_relayer_adapter_rust = "3.1.0"   # update to lastest version
...
```

### 2. Declare Static Clients

```rust
//  src/config.rs

use dotenvy::dotenv;
use solana_relayer_adapter_rust::{Astralane, BlockRazor, Jito, NextBlock, Nozomi, ZeroSlot};
use std::env;
use tokio::sync::OnceCell;

pub static NOZOMI_CLIENT: OnceCell<Nozomi> = OnceCell::const_new();
pub static ZSLOT_CLIENT: OnceCell<ZeroSlot> = OnceCell::const_new();
pub static JITO_CLIENT: OnceCell<Jito> = OnceCell::const_new();
pub static BRAZOR_CLIENT: OnceCell<BlockRazor> = OnceCell::const_new();
pub static ASTRA_CLIENT: OnceCell<Astralane> = OnceCell::const_new();
pub static NEXTBLOCK_CLIENT: OnceCell<NextBlock> = OnceCell::const_new();
```

### 3. Declare Clients Initialize Function

```rust
//  src/config.rs or other directory

pub async fn init_nozomi() {
    dotenv().ok();

    let nozomi_api_key = env::var("NOZOMI_API_KEY").expect("NOZOMI_API_KEY not set in .env");

    let nozomi = Nozomi::new_auto(nozomi_api_key).await;
    nozomi.health_check(50);
    NOZOMI_CLIENT.set(nozomi).unwrap();
}

pub async fn init_jito() {
    let jito = Jito::new_auto(None).await;
    JITO_CLIENT.set(jito).unwrap();
}

pub async fn init_nextblock() {
    dotenv().ok();

    let nextblock_api_key = env::var("NEXTBLOCK_API_KEY").expect("NEXTBLOCK_API_KEY not set in .env");

    let nextblock = NextBlock::new_auto(nextblock_api_key).await;
    NEXTBLOCK_CLIENT.set(nextblock).unwrap();
}

// Repeat for other providers...
```

### 4. Declare Clients with Region / Automatic Select
```rust
// src/main.rs or entry_point

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    init_nextblock().await;
    init_astra().await;
    init_nozomi().await;
    init_jito().await;
    ...

    Ok(())
}

```

### 5. Static Access to client
```rust
let client = JITO_CLIENT.get().ok_or("Jito not init").unwrap();
let client = NOZOMI_CLIENT.get().ok_or("Nozomi not init").unwrap();
...

let cu: u64;
let priority_fee_micro_lamport: u64;
let PRIVATE_KEY: Keypair;
let raw_ixs: Vec<Instruction>;
let tip_addr_idx : u8;
let third_party_fee: f64


let ixs = client.add_tip_ix(Tips {
    cu: Some(cu),
    priority_fee_micro_lamport: Some(priority_fee_micro_lamport),
    payer: PRIVATE_KEY.pubkey(),
    pure_ix: raw_ixs,
    tip_addr_idx: tip_addr_idx,
    tip_sol_amount: third_party_fee,
});

let result = client
    .send_transaction(&encoded)
    .await
    .map(|v| v.to_string())
    .map_err(|e| e.to_string())
```

---

## 🌍 Ping & Latency Diagnostics

- ### LOG

```
Nextblock-Fra                  (64.130.50.52)                    0.280 ms
Nextblock-Fra                  (64.130.50.52)                    0.160 ms
Connecting with Nextblock-Fra ...
Nextblock-Fra                  (64.130.50.52)                    0.183 ms
Astra-San Francisco            (45.32.86.58)                   146.506 ms
Astra-Amsterdam                (64.130.43.43)                    5.894 ms
Astra-Frankfurt                (162.19.222.232)                  8.101 ms
Astra-NY                       (173.231.40.226)                 84.369 ms
Astra-Tokyo                    (173.231.40.226)                 84.414 ms
Connecting with Astra-Amsterdam ...
Astra-Amsterdam                (64.130.43.43)                    5.917 ms
BlockRazor-NewYork             (185.209.179.15)                 82.583 ms
BlockRazor-Frankfurt           (64.130.32.137)                   0.135 ms
BlockRazor-AMS                 (64.130.43.53)                    5.914 ms
BlockRazor-Tokyo               (52.198.190.162)                257.519 ms
Connecting with BlockRazor-Frankfurt ...
BlockRazor-Frankfurt           (64.130.32.137)                   0.117 ms
Nozomi-PittDirect              (204.16.247.116)                 90.990 ms
Nozomi-TyoDirect               (202.8.9.153)                   258.808 ms
Nozomi-SgDirect                (202.8.11.174)                  160.986 ms
Nozomi-EwrDirect               (64.239.117.13)                  79.852 ms
Nozomi-AmsDirect               (64.130.55.186)                   5.804 ms
Nozomi-FraDirect               (64.239.35.13)                    0.104 ms
Nozomi-AmsSecure               (64.130.55.186)                   5.806 ms
Nozomi-TyoSecure               (202.8.9.153)                   258.736 ms
Nozomi-SgSecure                (202.8.11.174)                  160.976 ms
Nozomi-EwrSecure               (64.239.117.13)                  79.854 ms
Nozomi-PittSecure              (204.16.247.116)                 90.992 ms
Nozomi-FraSecure               (64.239.35.13)                    0.138 ms
Connecting with Nozomi-FraDirect ...
Nozomi-FraDirect               (64.239.35.13)                    0.066 ms
Jito-Mainnet                   (64.130.50.46)                    0.111 ms
Jito-Amsterdam                 (64.130.52.157)                   5.842 ms
Jito-Frankfurt                 (64.130.57.104)                   0.100 ms
Nozomi-FraDirect Health Check Successful
Jito-London                    (88.211.250.124)                 16.183 ms
Jito-NY                        (64.130.59.205)                  79.847 ms
Jito-SLC                       (64.130.53.56)                  128.648 ms
Jito-SG                        (202.8.11.169)                  151.699 ms
Jito-Tokyo                     (64.130.49.118)                 258.914 ms
Connecting with Jito-Frankfurt ...
Jito-Frankfurt                 (64.130.57.104)                   0.162 ms
ZeroSlot-NewYork               (172.66.40.254)                   0.765 ms
ZeroSlot-Frankfurt             (172.66.43.2)                     0.839 ms
ZeroSlot-AMS                   (172.66.43.2)                     0.857 ms
ZeroSlot-LA                    (172.66.40.254)                   0.816 ms
ZeroSlot-Tokyo                 (172.66.40.254)                   0.739 ms
Connecting with ZeroSlot-Tokyo ...
ZeroSlot-Tokyo                 (172.66.40.254)                   0.771 ms
```

- ### RESULT

### ✅ Fastest per Provider

| Provider       | Best Region     | Latency (ms) |
|----------------|----------------|--------------|
| **Nozomi**     | Frankfurt       | 0.066        |
| **Jito**       | Frankfurt       | 0.100        |
| **BlockRazor** | Frankfurt       | 0.117        |
| **NextBlock**  | Frankfurt       | 0.160        |
| **ZeroSlot**   | Tokyo           | 0.739        |
| **Astralane**  | Frankfurt       | 8.101        |

📌 The system will auto-select the optimal endpoint based on RTT.

---

## 📁 .env File

```env
NOZOMI_API_KEY=your_nozomi_key
ZERO_SLOT_KEY=your_zeroslot_key
BLOCKRAZOR_API_KEY=your_brazor_key
ASTRALANE_API_KEY=your_astralane_key
NEXTBLOCK_API_KEY=your_nextblock_key
```

---

## 🧠 Supported Providers Overview

| Provider     | Endpoint Management | Docs |
|--------------|---------------------|------|
| **Jito**      | Auto-region select   | [🔗](https://docs.jito.wtf/lowlatencytxnsend/) |
| **Nozomi**    | Direct vs Secure     | [🔗](https://use.temporal.xyz/nozomi/transaction-submission) |
| **ZeroSlot**  | Ultra-low RTT        | [🔗](https://0slot.trade/docs.php) |
| **BloxRoute** | Trader API support   | [🔗](https://docs.bloxroute.com/solana/trader-api/best-performance-for-landing-transactions) |
| **NextBlock** | High-speed nodes     | [🔗](https://docs.nextblock.io/getting-started/quickstart) |
| **Blockrazor**| Rust-native adapter  | [🔗](https://blockrazor.gitbook.io/blockrazor/solana/send-transaction/rust) |
| **Astralane** | Secure latency config| [🔗](https://astralane.gitbook.io/docs/low-latency/endpoints-and-configs) |

---

## 🧪 Example Health Check

```rust
let nozomi = NOZOMI_CLIENT.get().unwrap();
nozomi.health_check(50);
```

---

## 🤝 Contribution

PRs are welcome! Please open issues for feature requests or bug reports.

---

## 📜 License

MIT © @vvizardev
