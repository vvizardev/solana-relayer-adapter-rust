use crate::NozomiEndpoint;

#[derive(Debug, PartialEq, Clone)]
pub enum NozomiRegionsType {
    PittDirect,
    TyoDirect,
    SgDirect,
    EwrDirect,
    AmsDirect,
    FraDirect,
    AmsSecure,
    TyoSecure,
    SgSecure,
    EwrSecure,
    PittSecure,
    FraSecure,
}

pub const NOZOMI_REGIONS: &[NozomiEndpoint] = &[
    NozomiEndpoint {
        relayer: NozomiRegionsType::PittDirect,
        relayer_name: "Nozomi-PittDirect",
        submit_endpoint: "http://pit1.nozomi.temporal.xyz/?c=",
        ping_endpoint: "pit1.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::TyoDirect,
        relayer_name: "Nozomi-TyoDirect",
        submit_endpoint: "http://tyo1.nozomi.temporal.xyz/?c=",
        ping_endpoint: "tyo1.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::SgDirect,
        relayer_name: "Nozomi-SgDirect",
        submit_endpoint: "http://sgp1.nozomi.temporal.xyz/?c=",
        ping_endpoint: "sgp1.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::EwrDirect,
        relayer_name: "Nozomi-EwrDirect",
        submit_endpoint: "http://ewr1.nozomi.temporal.xyz/?c=",
        ping_endpoint: "ewr1.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::AmsDirect,
        relayer_name: "Nozomi-AmsDirect",
        submit_endpoint: "http://ams1.nozomi.temporal.xyz/?c=",
        ping_endpoint: "ams1.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::FraDirect,
        relayer_name: "Nozomi-FraDirect",
        submit_endpoint: "http://fra2.nozomi.temporal.xyz/?c=",
        ping_endpoint: "fra2.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::AmsSecure,
        relayer_name: "Nozomi-AmsSecure",
        submit_endpoint: "https://ams1.secure.nozomi.temporal.xyz/?c=",
        ping_endpoint: "ams1.secure.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::TyoSecure,
        relayer_name: "Nozomi-TyoSecure",
        submit_endpoint: "http://tyo1.secure.nozomi.temporal.xyz/?c=",
        ping_endpoint: "tyo1.secure.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::SgSecure,
        relayer_name: "Nozomi-SgSecure",
        submit_endpoint: "http://sgp1.secure.nozomi.temporal.xyz/?c=",
        ping_endpoint: "sgp1.secure.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::EwrSecure,
        relayer_name: "Nozomi-EwrSecure",
        submit_endpoint: "https://ewr1.secure.nozomi.temporal.xyz/?c=",
        ping_endpoint: "ewr1.secure.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::PittSecure,
        relayer_name: "Nozomi-PittSecure",
        submit_endpoint: "https://pit1.secure.nozomi.temporal.xyz/?c=",
        ping_endpoint: "pit1.secure.nozomi.temporal.xyz",
    },
    NozomiEndpoint {
        relayer: NozomiRegionsType::FraSecure,
        relayer_name: "Nozomi-FraSecure",
        submit_endpoint: "http://fra2.secure.nozomi.temporal.xyz/?c=",
        ping_endpoint: "fra2.secure.nozomi.temporal.xyz",
    },
];
