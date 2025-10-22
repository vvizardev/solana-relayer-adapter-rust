use crate::AstraEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum AstraRegionsType {
    Amsterdam,
    Frankfurt,
    LA,
    NY,
    Tokyo,
    Limburg,
    Paladine,
}

pub const ASTRA_REGIONS: &[AstraEndpoint] = &[
    AstraEndpoint {
        relayer: AstraRegionsType::LA,
        relayer_name: "Astra-San Francisco",
        submit_endpoint: "http://la.gateway.astralane.io/iris",
        ping_endpoint: "la.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::Amsterdam,
        relayer_name: "Astra-Amsterdam",
        submit_endpoint: "http://ams.gateway.astralane.io/iris",
        ping_endpoint: "ams.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::Frankfurt,
        relayer_name: "Astra-Frankfurt",
        submit_endpoint: "http://fr.gateway.astralane.io/iris",
        ping_endpoint: "fr.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::NY,
        relayer_name: "Astra-NY",
        submit_endpoint: "http://ny.gateway.astralane.io/iris",
        ping_endpoint: "ny.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::Tokyo,
        relayer_name: "Astra-Tokyo",
        submit_endpoint: "http://jp.gateway.astralane.io/iris",
        ping_endpoint: "jp.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::Limburg,
        relayer_name: "Astra-Limburg",
        submit_endpoint: "http://lim.gateway.astralane.io/iris",
        ping_endpoint: "lim.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::Paladine,
        relayer_name: "Astra-Paladine",
        submit_endpoint: "http://paladin.astralane.io/api/palidators",
        ping_endpoint: "paladin.astralane.io",
    },
];
