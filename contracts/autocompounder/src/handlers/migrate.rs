use cosmwasm_std::{DepsMut, Env, Response};
use forty_two::autocompounder::AutocompounderMigrateMsg;

use crate::contract::{AutocompounderApp, AutocompounderResult};

/// Unused for now but provided here as an example
/// Contract version is migrated automatically
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    _app: AutocompounderApp,
    _msg: AutocompounderMigrateMsg,
) -> AutocompounderResult {
    Ok(Response::default())
}
