use crate::CwStakingProvider;
use crate::error::StakingError;
use crate::traits::identify::Identify;

#[cfg(feature = "juno")]
pub use crate::providers::junoswap::{JunoSwap, JUNOSWAP};

#[cfg(any(feature = "juno", feature = "terra"))]
pub use crate::providers::loop_dex::{Loop, LOOP};

#[cfg(feature = "terra")]
pub use crate::providers::terraswap::{Terraswap, TERRASWAP};

#[cfg(any(feature = "juno", feature = "osmosis"))]
pub use crate::providers::osmosis::{Osmosis, OSMOSIS};

/// Given the provider name, return the *identified* provider implementation
pub(crate) fn resolve_provider_by_name(name: &str) -> Result<&'static dyn Identify, StakingError> {
    match name {
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
        _ => Err(StakingError::UnknownDex(name.to_owned())),
    }
}

/// Given the provider name, return the local provider implementation
pub(crate) fn resolve_local_provider(name: &str) -> Result<&'static dyn CwStakingProvider, StakingError> {
    match name {
        #[cfg(feature = "juno")]
        JUNOSWAP => Ok(&JunoSwap {}),
        #[cfg(any(feature = "juno", feature = "terra"))]
        LOOP => Ok(&Loop {}),
        #[cfg(feature = "terra")]
        TERRASWAP => Ok(&Terraswap {}),
        _ => Err(StakingError::ForeignDex(name.to_owned())),
    }
}
