use crate::ZSlotEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum ZSlotRegionsType {
    Frankfurt,
    NewYork,
    AMS,
    LA,
    Tokyo,
}

pub const ZSLOT_REGIONS: &[ZSlotEndpoint] = &[
    ZSlotEndpoint {
        relayer: ZSlotRegionsType::NewYork,
        relayer_name: "ZeroSlot-NewYork",
        submit_endpoint: "https://ny.0slot.trade?api-key=",
        ping_endpoint: "ny.0slot.trade",
    },
    ZSlotEndpoint {
        relayer: ZSlotRegionsType::Frankfurt,
        relayer_name: "ZeroSlot-Frankfurt",
        submit_endpoint: "https://de.0slot.trade?api-key=",
        ping_endpoint: "de.0slot.trade",
    },
    ZSlotEndpoint {
        relayer: ZSlotRegionsType::AMS,
        relayer_name: "ZeroSlot-AMS",
        submit_endpoint: "https://ams.0slot.trade?api-key=",
        ping_endpoint: "ams.0slot.trade",
    },
    ZSlotEndpoint {
        relayer: ZSlotRegionsType::LA,
        relayer_name: "ZeroSlot-LA",
        submit_endpoint: "https://la.0slot.trade?api-key=",
        ping_endpoint: "la.0slot.trade",
    },
    ZSlotEndpoint {
        relayer: ZSlotRegionsType::Tokyo,
        relayer_name: "ZeroSlot-Tokyo",
        submit_endpoint: "https://jp.0slot.trade?api-key=",
        ping_endpoint: "jp.0slot.trade",
    },
];
