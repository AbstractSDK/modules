//! # App Autocompounder
//!
//! `your_namespace::autocompounder` is an app which allows users to ...
//!
//! ## Creation
//! The contract can be added on an OS by calling [`ExecuteMsg::CreateModule`](crate::manager::ExecuteMsg::CreateModule) on the manager of the os.
//! ```ignore
//! let autocompounder_init_msg = InstantiateMsg::AutocompounderInstantiateMsg{
//!               /// The initial value for max_count
//!               pub max_count: Uint128,
//!               /// Initial user counts
//!               pub initial_counts: Option<Vec<(String, Uint128)>>,
//!           };
//!
//! let create_module_msg = ExecuteMsg::CreateModule {
//!                 module: Module {
//!                     info: ModuleInfo {
//!                         name: AUTOCOMPOUNDER.into(),
//!                         version: None,
//!                     },
//!                     kind: crate::core::modules::ModuleKind::External,
//!                 },
//!                 init_msg: Some(to_binary(&autocompounder_init_msg).unwrap()),
//!        };
//! // Call create_module_msg on manager
//! ```
//!
//! ## Migration
//! Migrating this contract is done by calling `ExecuteMsg::Upgrade` on [`crate::manager`] with `crate::AUTOCOMPOUNDER` as module.

use cosmwasm_std::{Uint128};

pub const AUTOCOMPOUNDER: &str = "4t2:autocompounder";

pub mod state {
    use cosmwasm_std::{Uint128};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use cw_storage_plus::{Item};

    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
    pub struct FeeConfig {
        pub performance: Uint128,
        pub deposit: Uint128,
        pub withdrawal: Uint128,
    }


    pub const FEE_CONFIG: Item<FeeConfig> = Item::new("fees");
}

/// Migrate msg
#[cosmwasm_schema::cw_serde]
pub struct AutocompounderMigrateMsg {}

/// Init msg
#[cosmwasm_schema::cw_serde]
pub struct AutocompounderInstantiateMsg {}

#[cosmwasm_schema::cw_serde]
pub enum AutocompounderExecuteMsg {
    UpdateFeeConfig {
        performance: Option<Uint128>,
        deposit: Option<Uint128>,
        withdrawal: Option<Uint128>,
    },
    Zap {},
    Compound {},

}

#[cosmwasm_schema::cw_serde]
pub enum AutocompounderQueryMsg {
    FeeConfig {}
}

