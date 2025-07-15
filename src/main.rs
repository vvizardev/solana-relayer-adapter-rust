use solana_relayer_adapter_rust::{Jito, Nozomi, ZeroSlot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let jito_client = Jito::new_auto(None).await;
    Nozomi::new_auto("4516a74a-ad06-4faf-9de4-10cce6e37f6b".to_string()).await;
    ZeroSlot::new_auto("4516a74a-ad06-4faf-9de4-10cce6e37f6b".to_string()).await;

    Ok(())
}
