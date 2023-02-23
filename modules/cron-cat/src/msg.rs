

//! # Tendermint Staking Api
//!
//! `abstract_os::tendermint_staking` exposes all the function of [`cosmwasm_std::CosmosMsg::Staking`] and [`cosmwasm_std::CosmosMsg::Distribution`].

use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Uint128;

use crate::api::{self};

pub type ExecuteMsg = api::ExecuteMsg<TendermintStakingExecuteMsg>;
pub type QueryMsg = api::QueryMsg<TendermintStakingQueryMsg>;

impl api::ApiExecuteMsg for TendermintStakingExecuteMsg {}
impl api::ApiQueryMsg for TendermintStakingQueryMsg {}

#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "boot", derive(boot_core::ExecuteFns))]
#[cfg_attr(feature = "boot", impl_into(ExecuteMsg))]
pub enum CronCatExecuteMsg {
    Register {
        
    }
}

/// Staking queries are available on [`cosmwasm_std::QuerierWrapper`] through [`cosmwasm_std::Deps`]. Helper function are exposed by [`abstract_sdk::tendermint_staking`]
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses)]
#[cfg_attr(feature = "boot", derive(boot_core::QueryFns))]
#[cfg_attr(feature = "boot", impl_into(QueryMsg))]
pub enum CronCatQueryMsg {}
