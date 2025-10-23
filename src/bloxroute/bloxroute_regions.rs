use crate::BxRouteEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum BxRouteRegionsType {
    Uk,
    Ny,
    Global,
    La,
    Ff,
    Ams,
    Ty,
}

macro_rules! bloxroute_endpoint {
    ($region:ident, $name:expr, $submit:expr, $ping:expr) => {
        BxRouteEndpoint {
            relayer: BxRouteRegionsType::$region,
            relayer_name: $name,
            submit_endpoint: $submit,
            ping_endpoint: $ping,
        }
    };
}

pub const BXROUTE_REGIONS: &[BxRouteEndpoint] = &[
    bloxroute_endpoint!(
        Uk,
        "BloxRoute-Uk",
        "http://uk.solana.dex.blxrbdn.com",
        "uk.solana.dex.blxrbdn.com"
    ),
    bloxroute_endpoint!(
        Ny,
        "BloxRoute-Ny",
        "http://ny.solana.dex.blxrbdn.com",
        "ny.solana.dex.blxrbdn.com"
    ),
    bloxroute_endpoint!(
        Global,
        "BloxRoute-Global",
        "http://global.solana.dex.blxrbdn.com",
        "global.solana.dex.blxrbdn.com"
    ),
    bloxroute_endpoint!(
        La,
        "BloxRoute-La",
        "http://la.solana.dex.blxrbdn.com",
        "la.solana.dex.blxrbdn.com"
    ),
    bloxroute_endpoint!(
        Ff,
        "BloxRoute-Ff",
        "http://germany.solana.dex.blxrbdn.com",
        "germany.solana.dex.blxrbdn.com"
    ),
    bloxroute_endpoint!(
        Ams,
        "BloxRoute-Ams",
        "http://amsterdam.solana.dex.blxrbdn.com",
        "amsterdam.solana.dex.blxrbdn.com"
    ),
    bloxroute_endpoint!(
        Ty,
        "BloxRoute-Ty",
        "http://tokyo.solana.dex.blxrbdn.com",
        "tokyo.solana.dex.blxrbdn.com"
    ),
];
