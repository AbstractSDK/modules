use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

use forty_two::autocompounder::{AUTOCOMPOUNDER, AutocompounderInstantiateMsg};
use forty_two::autocompounder::state::{ FEE_CONFIG, FeeConfig};

use crate::contract::{AutocompounderApp, AutocompounderResult};

/// Initial instantiation of the contract
pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: AutocompounderApp,
    _msg: AutocompounderInstantiateMsg,
) -> AutocompounderResult {
    let config: FeeConfig = FeeConfig {
        performance: Uint128::zero(),
        deposit: Uint128::zero(),
        withdrawal: Uint128::zero(),
    };

    FEE_CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("contract", AUTOCOMPOUNDER))
}
