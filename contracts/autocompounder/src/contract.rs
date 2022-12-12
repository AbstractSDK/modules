use abstract_app::export_endpoints;
use abstract_app::AppContract;
use abstract_sdk::os::EXCHANGE;
use cosmwasm_std::Response;
use forty_two::autocompounder::{AUTOCOMPOUNDER, AutocompounderExecuteMsg, AutocompounderInstantiateMsg, AutocompounderMigrateMsg, AutocompounderQueryMsg};
use forty_two::cw_staking::CW_STAKING;

use crate::error::AutocompounderError;
use crate::handlers::{self};

// As an app writer, the only changes necessary to this file are with the handlers and API dependencies on the `AUTOCOMPOUNDER_APP` const.
pub type AutocompounderApp = AppContract<
    AutocompounderError,
    AutocompounderExecuteMsg,
    AutocompounderInstantiateMsg,
    AutocompounderQueryMsg,
    AutocompounderMigrateMsg,
>;

pub type AutocompounderResult = Result<Response, AutocompounderError>;

/// The initial version of the app, which will use the package version if not altered
const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Expected replies
pub const EXAMPLE_REPLY_ID: u64 = 69420;

/// Used as the foundation for building your app.
/// All entrypoints are executed through this const (`instantiate`, `query`, `execute`, `migrate`)
/// The `dependencies` are Abstract API dependencies in the format: Vec(`namespace:contract_name`)
const APP: AutocompounderApp = AutocompounderApp::new(AUTOCOMPOUNDER, MODULE_VERSION)
    .with_instantiate(handlers::instantiate_handler)
    .with_query(handlers::query_handler)
    .with_execute(handlers::execute_handler)
    .with_migrate(handlers::migrate_handler)
    .with_replies(&[(EXAMPLE_REPLY_ID, handlers::example_reply_handler)])
    .with_dependencies(&[EXCHANGE, CW_STAKING]);

// don't export endpoints when imported as library
#[cfg(not(feature = "library"))]
// Export the endpoints for this contract
export_endpoints!(APP, AutocompounderApp);
