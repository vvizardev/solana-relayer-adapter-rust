use crate::NextBlockEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum NextBlockRegionsType {
    Fra,
    NY,
}

macro_rules! nextblock_endpoint {
    ($region:ident, $name:expr, $submit:expr, $ping:expr) => {
        NextBlockEndpoint {
            relayer: NextBlockRegionsType::$region,
            relayer_name: $name,
            submit_endpoint: $submit,
            ping_endpoint: $ping,
        }
    };
}

pub const NEXTBLOCK_REGIONS: &[NextBlockEndpoint] = &[
    nextblock_endpoint!(
        Fra,
        "Nextblock-Fra",
        "https://fra.nextblock.io/api/v2/submit",
        "fra.nextblock.io"
    ),
    nextblock_endpoint!(
        NY,
        "Nextblock-Fra",
        "https://fra.nextblock.io/api/v2/submit",
        "fra.nextblock.io"
    ),
];
