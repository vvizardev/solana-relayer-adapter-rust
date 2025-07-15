use crate::BRazorEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum BRazorRegionsType {
    Frankfurt,
    NewYork,
    AMS,
    Tokyo,
}

macro_rules! blockrazor_endpoint {
    ($region:ident, $name:expr, $submit:expr, $ping:expr) => {
        BRazorEndpoint {
            relayer: BRazorRegionsType::$region,
            relayer_name: $name,
            submit_endpoint: $submit,
            ping_endpoint: $ping,
        }
    };
}

pub const BRAZOR_REGIONS: &[BRazorEndpoint] = &[
    blockrazor_endpoint!(
        NewYork,
        "ZeroSlot-NewYork",
        "http://newyork.solana.blockrazor.xyz:443/sendTransaction",
        "newyork.solana.blockrazor.xyz"
    ),
    blockrazor_endpoint!(
        Frankfurt,
        "ZeroSlot-Frankfurt",
        "http://frankfurt.solana.blockrazor.xyz:443/sendTransaction",
        "frankfurt.solana.blockrazor.xyz"
    ),
    blockrazor_endpoint!(
        AMS,
        "ZeroSlot-AMS",
        "http://amsterdam.solana.blockrazor.xyz:443/sendTransaction",
        "amsterdam.solana.blockrazor.xyz"
    ),
    blockrazor_endpoint!(
        Tokyo,
        "ZeroSlot-Tokyo",
        "http://tokyo.solana.blockrazor.xyz:443/sendTransaction",
        "tokyo.solana.blockrazor.xyz"
    ),
];
