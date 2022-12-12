use crate::error::StakingError;
use crate::CwStakingProvider;
use crate::staking_trait::Identify;

#[cfg(feature = "juno")]
pub use crate::providers::junoswap::{JunoSwap, JUNOSWAP};

#[cfg(any(feature = "juno", feature = "terra"))]
pub use crate::providers::loop_dex::{Loop, LOOP};

#[cfg(feature = "terra")]
pub use crate::providers::terraswap::{Terraswap, TERRASWAP};

#[cfg(any(feature = "juno", feature = "osmosis"))]
pub use crate::providers::osmosis::{Osmosis, OSMOSIS};

/// Given the exchange name, return the *identified* provider implementation
pub(crate) fn identify_provider(value: &str) -> Result<&'static dyn Identify, StakingError> {
    match value {
        #[cfg(feature = "juno")]
        JUNOSWAP => Ok(&JunoSwap {}),
        #[cfg(feature = "juno")]
        OSMOSIS => Ok(&Osmosis {
            local_proxy_addr: None,
        }),
        #[cfg(any(feature = "juno", feature = "terra"))]
        LOOP => Ok(&Loop {}),
        #[cfg(feature = "terra")]
        TERRASWAP => Ok(&Terraswap {}),
        _ => Err(StakingError::UnknownDex(value.to_owned())),
    }
}

/// Given the exchange name, return the local provider implementation
pub(crate) fn resolve_local_provider(value: &str) -> Result<&'static dyn CwStakingProvider, StakingError> {
    match value {
        #[cfg(feature = "juno")]
        JUNOSWAP => Ok(&JunoSwap {}),
        #[cfg(any(feature = "juno", feature = "terra"))]
        LOOP => Ok(&Loop {}),
        #[cfg(feature = "terra")]
        TERRASWAP => Ok(&Terraswap {}),
        _ => Err(StakingError::ForeignDex(value.to_owned())),
    }
}
