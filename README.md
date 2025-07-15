v.1. Jito
- https://docs.jito.wtf/lowlatencytxnsend/

v.2. Nozomi
- https://use.temporal.xyz/nozomi/transaction-submission

v.3. ZeroSlot
- https://0slot.trade/docs.php

v.3.1. BloxRoute
- https://docs.bloxroute.com/solana/trader-api/best-performance-for-landing-transactions

v.3.2 NextBlock
- https://docs.nextblock.io/getting-started/quickstart

v.3.3 Blockrazor
- https://blockrazor.gitbook.io/blockrazor/solana/send-transaction/rust

v.3.4 Astralane
- https://astralane.gitbook.io/docs/low-latency/endpoints-and-configs


```
use solana_relayer_adapter_rust::{Jito, Nozomi, ZeroSlot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jito_client_free_client = Jito::new_auto(None).await;
    let jito_client_premium_client = Jito::new_auto(Some("jito-api-key")).await;
    let nozomi_client = Nozomi::new_auto("nozomi-api-key".to_string()).await;
    let zslot_client = ZeroSlot::new_auto("zslot-api-key".to_string()).await;

    Ok(())
}
```

```
Jito-Mainnet                   (64.130.33.116)                  20.845 ms
Jito-Amsterdam                 (64.130.52.73)                  143.615 ms
Jito-Frankfurt                 (64.130.50.46)                  145.726 ms
Jito-London                    (88.211.250.124)                134.836 ms
Jito-NY                        (64.130.51.62)                   61.534 ms
Jito-SLC                       (64.130.53.56)                   21.152 ms
Jito-SG                        (202.8.11.169)                  174.565 ms
Jito-Tokyo                     (64.130.49.134)                  98.785 ms
Connecting with Jito-Mainnet ...
Jito-Mainnet                   (64.130.53.56)                   20.952 ms
Nozomi-PittDirect              (204.16.247.116)                 55.279 ms
Nozomi-TyoDirect               (202.8.9.153)                   105.755 ms
Nozomi-SgDirect                (202.8.11.174)                  184.224 ms
Nozomi-EwrDirect               (64.239.117.13)                  72.324 ms
Nozomi-AmsDirect               (64.130.55.186)                 142.781 ms
Nozomi-FraDirect               (64.239.35.13)                  153.636 ms
Nozomi-AmsSecure               (64.130.55.186)                 141.262 ms
Nozomi-TyoSecure               (202.8.9.153)                   105.619 ms
Nozomi-SgSecure                (202.8.11.174)                  184.121 ms
Nozomi-EwrSecure               (64.239.117.13)                  75.385 ms
Nozomi-PittSecure              (204.16.247.116)                 55.154 ms
Nozomi-FraSecure               (64.239.35.13)                  156.602 ms
Connecting with Nozomi-PittSecure ...
Nozomi-PittSecure              (204.16.247.116)                 55.175 ms
ZeroSlot-NewYork               (2606:4700:3108::ac42:28fe)       1.065 ms
ZeroSlot-Frankfurt             (2606:4700:3108::ac42:28fe)       0.850 ms
ZeroSlot-AMS                   (2606:4700:3108::ac42:28fe)       0.915 ms
ZeroSlot-LA                    (2606:4700:3108::ac42:28fe)       0.919 ms
ZeroSlot-Tokyo                 (2606:4700:3108::ac42:28fe)       0.803 ms
Connecting with ZeroSlot-Tokyo ...
ZeroSlot-Tokyo                 (2606:4700:3108::ac42:28fe)       0.728 ms
```