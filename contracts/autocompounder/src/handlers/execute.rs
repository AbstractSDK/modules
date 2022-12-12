use cosmwasm_std::{DepsMut, Env, MessageInfo, Uint128};
use forty_two::autocompounder::AutocompounderExecuteMsg;

use crate::contract::{AutocompounderApp, AutocompounderResult};
use crate::error::AutocompounderError;

/// Handle the `AutocompounderExecuteMsg`s sent to this app.
pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    app: AutocompounderApp,
    msg: AutocompounderExecuteMsg,
) -> AutocompounderResult {
    match msg {
        AutocompounderExecuteMsg::UpdateFeeConfig { performance, withdrawal, deposit } => {
            update_fee_config(deps, info, app, performance, withdrawal, deposit)
        }
        _ => Err(AutocompounderError::ExceededMaxCount {}),
    }
}

/// Update the application configuration.
pub fn update_fee_config(
    deps: DepsMut,
    msg_info: MessageInfo,
    dapp: AutocompounderApp,
    _fee: Option<Uint128>,
    _withdrawal: Option<Uint128>,
    _deposit: Option<Uint128>,
) -> AutocompounderResult {
    dapp.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;

    unimplemented!()
}
