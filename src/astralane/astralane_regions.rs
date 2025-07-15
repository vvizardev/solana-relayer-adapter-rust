use crate::AstraEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum AstraRegionsType {
    Amsterdam,
    Frankfurt,
    LAX,
    NY,
    Tokyo,
}

pub const ASTRA_REGIONS: &[AstraEndpoint] = &[
    AstraEndpoint {
        relayer: AstraRegionsType::LAX,
        relayer_name: "Astra-San Francisco",
        submit_endpoint: "https://lax.gateway.astralane.io/iris?api-key=",
        ping_endpoint: "lax.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::Amsterdam,
        relayer_name: "Astra-Amsterdam",
        submit_endpoint: "http://ams.gateway.astralane.io/iris?api-key=",
        ping_endpoint: "ams.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::Frankfurt,
        relayer_name: "Astra-Frankfurt",
        submit_endpoint: "http://fr.gateway.astralane.io/iris?api-key=",
        ping_endpoint: "fr.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::NY,
        relayer_name: "Astra-NY",
        submit_endpoint: "http://ny.gateway.astralane.io/iris?api-key=",
        ping_endpoint: "ny.gateway.astralane.io",
    },
    AstraEndpoint {
        relayer: AstraRegionsType::Tokyo,
        relayer_name: "Astra-Tokyo",
        submit_endpoint: "http://ny.gateway.astralane.io/iris?api-key=",
        ping_endpoint: "ny.gateway.astralane.io",
    },
];
