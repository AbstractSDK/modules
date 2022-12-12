use abstract_extension::{export_endpoints, ExtensionContract};
use abstract_sdk::{
    base::endpoints::{ExecuteEndpoint, InstantiateEndpoint, QueryEndpoint},
};


use cosmwasm_std::{
    Empty, Response,
};
use cw_4t2::cw_staking::{CwStakingQueryMsg, CwStakingRequestMsg, CW_STAKING};

use crate::{error::StakingError, handlers};

const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type CwStakingExtension = ExtensionContract<StakingError, CwStakingRequestMsg, Empty, CwStakingQueryMsg>;
pub type CwStakingResult = Result<Response, StakingError>;

pub const CW_STAKING_EXTENSION: CwStakingExtension = CwStakingExtension::new(CW_STAKING, MODULE_VERSION)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler);


// don't export endpoints when imported as library
#[cfg(not(feature = "library"))]
// Export the endpoints for this contract
export_endpoints!(CW_STAKING_EXTENSION, CwStakingExtension);
