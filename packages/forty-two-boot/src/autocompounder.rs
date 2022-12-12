use abstract_os::app::{BaseExecuteMsg, BaseQueryMsg};
use abstract_os::{app};
use boot_core::{BootEnvironment, BootError, Contract, IndexResponse, TxResponse};
use boot_core::prelude::{boot_contract, BootExecute, BootQuery};
use cosmwasm_std::Coin;
use serde::de::DeserializeOwned;
use serde::Serialize;

use forty_two::autocompounder::{AutocompounderExecuteMsg, AutocompounderInstantiateMsg, AutocompounderMigrateMsg, AutocompounderQueryMsg};

type AppInstantiateMsg = app::InstantiateMsg<AutocompounderInstantiateMsg>;
type AppExecuteMsg = app::ExecuteMsg<AutocompounderExecuteMsg>;
type AppQueryMsg = app::QueryMsg<AutocompounderQueryMsg>;
type AppMigrateMsg = app::MigrateMsg<AutocompounderMigrateMsg>;

/// Contract wrapper for deploying with BOOT
#[boot_contract(AppInstantiateMsg, AppExecuteMsg, AppQueryMsg, AppMigrateMsg)]
pub struct AutocompounderApp<Chain>;

impl<Chain: BootEnvironment> AutocompounderApp<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        Self(
            Contract::new(name, chain).with_wasm_path("autocompounder"),
        )
    }

    /// Temporary helper to query the app explicitly
    pub fn query_app<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: AutocompounderQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&app::QueryMsg::App(query_msg))
    }

    /// Temporary helper to query the app base explicitly
    pub fn query_base<T: Serialize + DeserializeOwned>(
        &self,
        query_msg: BaseQueryMsg,
    ) -> Result<T, BootError> {
        self.query(&app::QueryMsg::Base(query_msg))
    }

    /// Temporary helper to execute the app explicitly
    pub fn execute_app(
        &self,
        execute_msg: AutocompounderExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&app::ExecuteMsg::App(execute_msg), coins)
    }

    /// Temporary helper to execute the app base explicitly
    pub fn execute_base(
        &self,
        execute_msg: BaseExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        self.execute(&app::ExecuteMsg::Base(execute_msg), coins)
    }
}
