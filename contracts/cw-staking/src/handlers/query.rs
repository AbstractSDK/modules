use cosmwasm_std::{Binary, Deps, Env, StdError, StdResult};
use forty_two::cw_staking::CwStakingQueryMsg;
use crate::contract::CwStakingExtension;

pub fn query_handler(_deps: Deps, _env: Env, _app: &CwStakingExtension, _msg: CwStakingQueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("Unknown query"))
}
