use boot_core::prelude::boot_contract;

use boot_core::{BootEnvironment, Contract, IndexResponse, TxResponse};


use cosmwasm_std::Empty;
use abstract_os::extension;
use forty_two::cw_staking::{CwStakingQueryMsg, CwStakingRequestMsg};

type ExtensionExecuteMsg = extension::ExecuteMsg<CwStakingRequestMsg>;
type ExtensionQueryMsg = extension::QueryMsg<CwStakingQueryMsg>;

/// Contract wrapper for interacting with BOOT
#[boot_contract(Empty, ExtensionExecuteMsg, ExtensionQueryMsg, Empty)]
pub struct CwStaking<Chain>;

/// implement chain-generic functions
impl<Chain: BootEnvironment> CwStaking<Chain> where TxResponse<Chain>: IndexResponse {
    pub fn new(id: &str, chain: &Chain) -> Self {
        Self(
            Contract::new(id, chain)
                .with_wasm_path("cw_staking"),
        )
    }
}
