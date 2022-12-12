//! # Staking Extension
//!
//! `4t2::cw-staking`

use abstract_sdk::os::objects::{AnsAsset, AssetEntry};
use cosmwasm_schema::QueryResponses;


pub type ProviderName = String;
pub type LpToken = AnsAsset;

/// The callback id for staking over ibc
pub const IBC_STAKING_PROVIDER_ID: u32 = 22335;

pub const CW_STAKING: &str = "4t2:cw_staking";

/// A request message that's sent to this staking extension
#[cosmwasm_schema::cw_serde]
pub struct CwStakingRequestMsg {
    pub provider: ProviderName,
    pub action: CwStakingAction,
}

#[cosmwasm_schema::cw_serde]
/// Possible actions to perform on the staking contract
pub enum CwStakingAction {
    /// Stake a given LP token
    Stake {
        lp_token: LpToken,
    },
    /// Unstake a given LP token
    Unstake {
        lp_token: LpToken,
    },
    /// Claim rewards for a given LP token
    Claim {
        lp_token_name: AssetEntry,
    },
}

#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses)]
pub enum CwStakingQueryMsg {
}
