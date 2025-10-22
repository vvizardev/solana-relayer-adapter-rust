use crate::NextBlockEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum NextBlockRegionsType {
    Fra,
    NY,
    Slc,
    Tokyo,
    London,
    Sgp,
    Ams,
    Vil,
    Dub,
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
        "https://fra.nextblock.io",
        "fra.nextblock.io"
    ),
    nextblock_endpoint!(
        NY,
        "Nextblock-Ny",
        "https://ny.nextblock.io",
        "ny.nextblock.io"
    ),
    nextblock_endpoint!(
        Slc,
        "Nextblock-Slc",
        "https://slc.nextblock.io",
        "slc.nextblock.io"
    ),
    nextblock_endpoint!(
        Tokyo,
        "Nextblock-Tokyo",
        "https://tokyo.nextblock.io",
        "tokyo.nextblock.io"
    ),
    nextblock_endpoint!(
        London,
        "Nextblock-London",
        "https://london.nextblock.io",
        "london.nextblock.io"
    ),
    nextblock_endpoint!(
        Sgp,
        "Nextblock-Sgp",
        "https://sgp.nextblock.io",
        "sgp.nextblock.io"
    ),
    nextblock_endpoint!(
        Ams,
        "Nextblock-Ams",
        "https://ams.nextblock.io",
        "ams.nextblock.io"
    ),
    nextblock_endpoint!(
        Vil,
        "Nextblock-Vil",
        "https://vilnius.nextblock.io",
        "vilnius.nextblock.io"
    ),
    nextblock_endpoint!(
        Dub,
        "Nextblock-Dub",
        "https://dublin.nextblock.io",
        "dublin.nextblock.io"
    ),
];
