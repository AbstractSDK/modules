pub mod contract;
pub mod error;
mod providers;

mod handlers;
mod traits;

use traits::*;

pub use local_cw_staking::LocalCwStaking;
pub use cw_staking_provider::CwStakingProvider;

#[cfg(any(feature = "juno", feature = "osmosis"))]
pub mod host_exchange {
    pub use super::providers::osmosis::Osmosis;
}

// #[cfg(test)]
// #[cfg(not(target_arch = "wasm32"))]
// mod tests;
