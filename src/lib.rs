pub(crate) mod commands;
pub mod contract;
pub(crate) mod staking_trait;
pub mod error;
mod providers;

// TODO: move elsewhere
pub mod cw_staking;
mod handlers;
mod provider_resolver;

pub use commands::LocalCwStaking;
pub use staking_trait::CwStakingProvider;

#[cfg(any(feature = "juno", feature = "osmosis"))]
pub mod host_exchange {
    pub use super::providers::osmosis::Osmosis;
}

// #[cfg(test)]
// #[cfg(not(target_arch = "wasm32"))]
// mod tests;
