use cosmwasm_std::{DepsMut, Env, Reply};

use crate::contract::{AutocompounderApp, AutocompounderResult};

pub fn example_reply_handler(
    _deps: DepsMut,
    _env: Env,
    _app: AutocompounderApp,
    _reply: Reply,
) -> AutocompounderResult {
    // Logic to execute on example reply
    todo!()
}
