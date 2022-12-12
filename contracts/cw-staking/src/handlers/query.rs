use cosmwasm_std::{Binary, Deps, Env, StdError, StdResult};
use cw_4t2::cw_staking::CwStakingQueryMsg;
use crate::contract::CwStakingExtension;

pub fn query_handler(_deps: Deps, _env: Env, _app: &CwStakingExtension, _msg: CwStakingQueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("Unknown query"))
}
