use cosmwasm_std::{Binary, Deps, Env, StdError, StdResult};
use crate::contract::CwStakingExtension;
use crate::cw_staking::CwStakingQueryMsg;

pub fn query_handler(_deps: Deps, _env: Env, _app: &CwStakingExtension, msg: CwStakingQueryMsg) -> StdResult<Binary> {
    match msg {
        _ => Err(StdError::generic_err("Unknown query")),
    }
}
