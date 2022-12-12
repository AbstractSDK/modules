use abstract_extension::{export_endpoints, ExtensionContract};
use abstract_sdk::{
    base::endpoints::{ExecuteEndpoint, InstantiateEndpoint, QueryEndpoint},
    feature_objects::AnsHost,
    IbcInterface, Resolve,
};
use abstract_sdk::base::features::AbstractNameService;
use abstract_sdk::os::{extension::{ExecuteMsg, InstantiateMsg, QueryMsg}, ibc_client::CallbackInfo, objects::AnsAsset};
use cosmwasm_std::{
    Binary, Coin, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult, to_binary,
};

use crate::{commands::LocalCwStaking, CwStakingProvider, error::StakingError, handlers, staking_trait::Identify};
use crate::cw_staking::{CwStakingAction, CwStakingQueryMsg, CwStakingRequestMsg, IBC_STAKING_PROVIDER_ID, ProviderName};

pub const MODULE_NAME: &str = "4t2:cw_staking";

const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type CwStakingExtension = ExtensionContract<StakingError, CwStakingRequestMsg, Empty, CwStakingQueryMsg>;
pub type CwStakingResult = Result<Response, StakingError>;

pub const CW_STAKING_EXTENSION: CwStakingExtension = CwStakingExtension::new(MODULE_NAME, MODULE_VERSION)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler);


// don't export endpoints when imported as library
#[cfg(not(feature = "library"))]
// Export the endpoints for this contract
export_endpoints!(CW_STAKING_EXTENSION, CwStakingExtension);
