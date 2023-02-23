use abstract_app::{export_endpoints, AppContract};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use abstract_sdk::os::tendermint_staking::TendermintStakingExecuteMsg;
use abstract_sdk::Execution;

use crate::error::CronCatError;
use crate::staking::*;

use abstract_sdk::os::TENDERMINT_STAKING;
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type CronCatApp = AppContract<CronCatError, TendermintStakingExecuteMsg>;
pub type CronCatResult = Result<Response, CronCatError>;

const STAKING_API: CronCatApp =
    CronCatApp::new(TENDERMINT_STAKING, CONTRACT_VERSION, None)
        .with_execute(handle_request);

// Export handlers
#[cfg(not(feature = "library"))]
export_endpoints!(STAKING_API, CronCatApp);

