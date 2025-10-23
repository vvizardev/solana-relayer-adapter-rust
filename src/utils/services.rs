use crate::*;

#[derive(Debug, Clone)]
pub struct NozomiEndpoint {
    pub relayer: NozomiRegionsType,
    pub submit_endpoint: &'static str,
    pub ping_endpoint: &'static str,
    pub relayer_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct JitoEndpoint {
    pub relayer: JitoRegionsType,
    pub submit_endpoint: &'static str,
    pub ping_endpoint: &'static str,
    pub relayer_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct ZSlotEndpoint {
    pub relayer: ZSlotRegionsType,
    pub submit_endpoint: &'static str,
    pub ping_endpoint: &'static str,
    pub relayer_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct BRazorEndpoint {
    pub relayer: BRazorRegionsType,
    pub submit_endpoint: &'static str,
    pub ping_endpoint: &'static str,
    pub relayer_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct BxRouteEndpoint {
    pub relayer: BxRouteRegionsType,
    pub submit_endpoint: &'static str,
    pub ping_endpoint: &'static str,
    pub relayer_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct AstraEndpoint {
    pub relayer: AstraRegionsType,
    pub submit_endpoint: &'static str,
    pub ping_endpoint: &'static str,
    pub relayer_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct NextBlockEndpoint {
    pub relayer: NextBlockRegionsType,
    pub submit_endpoint: &'static str,
    pub ping_endpoint: &'static str,
    pub relayer_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct HeliusEndpoint {
    pub relayer: HeliusRegionsType,
    pub submit_endpoint: &'static str,
    pub ping_endpoint: &'static str,
    pub relayer_name: &'static str,
}
