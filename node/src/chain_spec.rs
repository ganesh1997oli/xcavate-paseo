use parachain_template_runtime as runtime;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use crate::constant::xcavate;


/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    #[serde(alias = "relayChain", alias = "RelayChain")]
    pub relay_chain: String,
    /// The id of the Parachain.
    #[serde(alias = "paraId", alias = "ParaId")]
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

pub fn development_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), xcavate::TOKEN_SYMBOL.into());
    properties.insert("tokenDecimals".into(), xcavate::TOKEN_DECIMALS.into());
    properties.insert("ss58Format".into(), xcavate::SS58_FORMAT.into());

    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: xcavate::RELAY_CHAIN.into(),
            // You MUST set this to the correct network!
            para_id: xcavate::PARACHAIN_ID,
        },
    )
    .with_name("Xcavate Development")
    .with_id("dev")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    // .with_genesis_config_patch(patch)
    .build()
}

pub fn local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), xcavate::TOKEN_SYMBOL.into());
    properties.insert("tokenDecimals".into(), xcavate::TOKEN_DECIMALS.into());
    properties.insert("ss58Format".into(), xcavate::SS58_FORMAT.into());

    #[allow(deprecated)]
    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: xcavate::RELAY_CHAIN.into(),
            // You MUST set this to the correct network!
            para_id: xcavate::PARACHAIN_ID,
        },
    )
    .with_name("Local Testnet")
    .with_id("local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_preset_name(sc_chain_spec::LOCAL_TESTNET_RUNTIME_PRESET)
    .with_protocol_id("template-local")
    .with_properties(properties)
    .build()
}
