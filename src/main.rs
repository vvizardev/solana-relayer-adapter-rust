use solana_relayer_adapter_rust::{Jito, Nozomi, ZeroSlot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jito_client = Jito::new_auto(None).await;
    let jito_client = Jito::new_auto(Some("api-key".to_string())).await;
    let nozomi_client = Nozomi::new_auto("api-key".to_string()).await;
    let zslot_client = ZeroSlot::new_auto("api-key".to_string()).await;

    Ok(())
}
