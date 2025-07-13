use solana_relayer_adapter_rust::{Jito, Nozomi, ZeroSlot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jito_regions = Jito::ping_endpoints();
    Jito::icmp_ping_all(jito_regions).await;
    let nozomi_regions = Nozomi::ping_endpoints();
    Nozomi::icmp_ping_all(nozomi_regions).await;
    let nozomi_regions = ZeroSlot::ping_endpoints();
    ZeroSlot::icmp_ping_all(nozomi_regions).await;

    Ok(())
}
