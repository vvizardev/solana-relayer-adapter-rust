use crate::HeliusEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum HeliusRegionsType {
    Slc,
    Ewr,
    Lon,
    Fra,
    Ams,
    Sg,
    Tyo,
}

macro_rules! helius_endpoint {
    ($region:ident, $name:expr, $submit:expr, $ping:expr) => {
        HeliusEndpoint {
            relayer: HeliusRegionsType::$region,
            relayer_name: $name,
            submit_endpoint: $submit,
            ping_endpoint: $ping,
        }
    };
}

pub const HELIUS_REGIONS: &[HeliusEndpoint] = &[
    helius_endpoint!(
        Slc,
        "Helius-SaltLakeCity",
        "http://slc-sender.helius-rpc.com/fast",
        "slc-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Ewr,
        "Helius-Newark",
        "http://ewr-sender.helius-rpc.com/fast",
        "ewr-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Lon,
        "Helius-London",
        "http://lon-sender.helius-rpc.com/fast",
        "lon-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Fra,
        "Helius-Frankfurt",
        "http://fra-sender.helius-rpc.com/fast",
        "fra-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Ams,
        "Helius-Amsterdam",
        "http://ams-sender.helius-rpc.com/fast",
        "ams-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Sg,
        "Helius-Singapore",
        "http://sg-sender.helius-rpc.com/fast",
        "sg-sender.helius-rpc.com"
    ),
    helius_endpoint!(
        Tyo,
        "Helius-Tokyo",
        "http://tyo-sender.helius-rpc.com/fast",
        "tyo-sender.helius-rpc.com"
    )
];
