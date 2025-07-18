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

pub async fn init_nozomi() {
    dotenv().ok();

    let nozomi_api_key = env::var("NOZOMI_API_KEY").expect("NOZOMI_API_KEY not set in .env");

    let nozomi = Nozomi::new_auto(nozomi_api_key).await;
    nozomi.health_check(50);
    NOZOMI_CLIENT.set(nozomi).unwrap();
}

pub async fn init_zslot() {
    dotenv().ok();

    let zslot_api_key = env::var("ZERO_SLOT_KEY").expect("ZERO_SLOT_KEY not set in .env");

    let zslot = ZeroSlot::new_auto(zslot_api_key).await;
    ZSLOT_CLIENT.set(zslot).unwrap();
}

pub async fn init_jito() {
    let jito = Jito::new_auto(None).await;
    JITO_CLIENT.set(jito).unwrap();
}

pub async fn init_brazor() {
    dotenv().ok();

    let brazor_api_key =
        env::var("BLOCKRAZOR_API_KEY").expect("BLOCKRAZOR_API_KEY not set in .env");

    let brazor = BlockRazor::new_auto(brazor_api_key).await;
    BRAZOR_CLIENT.set(brazor).unwrap();
}

pub async fn init_astra() {
    dotenv().ok();

    let astra_api_key = env::var("ASTRALANE_API_KEY").expect("ASTRALANE_API_KEY not set in .env");

    let astra = Astralane::new_auto(astra_api_key).await;
    ASTRA_CLIENT.set(astra).unwrap();
}

pub async fn init_nextblock() {
    dotenv().ok();

    let nextblock_api_key =
        env::var("NEXTBLOCK_API_KEY").expect("NEXTBLOCK_API_KEY not set in .env");

    let nextblock = NextBlock::new_auto(nextblock_api_key).await;
    NEXTBLOCK_CLIENT.set(nextblock).unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_nextblock().await;
    init_astra().await;
    init_brazor().await;
    init_nozomi().await;
    init_jito().await;
    init_zslot().await;

    Ok(())
}
