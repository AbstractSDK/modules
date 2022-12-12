use crate::contract::AutocompounderApp;
use cosmwasm_std::{to_binary,  Binary, Deps, Env, StdResult};
use forty_two::autocompounder::AutocompounderQueryMsg;
use forty_two::autocompounder::state::{FEE_CONFIG, FeeConfig};

const _DEFAULT_PAGE_SIZE: u8 = 5;
const _MAX_PAGE_SIZE: u8 = 20;

/// Handle queries sent to this app.
pub fn query_handler(
    deps: Deps,
    _env: Env,
    _app: &AutocompounderApp,
    msg: AutocompounderQueryMsg,
) -> StdResult<Binary> {
    match msg {
        AutocompounderQueryMsg::FeeConfig { } => to_binary(&query_fee_config(deps)?)?,
    };
    unimplemented!();
}

/// Returns the current configuration.
pub fn query_fee_config(deps: Deps) -> StdResult<FeeConfig> {
    let _config = FEE_CONFIG.load(deps.storage)?;

    unimplemented!();
}
