use abstract_sdk::base::features::AbstractNameService;
use abstract_sdk::os::objects::AnsAsset;
use abstract_sdk::Execution;
use cosmwasm_std::{CosmosMsg, Decimal, Deps, DepsMut, ReplyOn, SubMsg};
use cw_asset::Asset;

use crate::{error::StakingError, CwStakingProvider};
use abstract_sdk::os::{
    objects::{AssetEntry, UncheckedContractEntry},
};
use crate::cw_staking::{CwStakingAction, LpToken};

pub const STAKE_REPLY_ID: u64 = 8542;
pub const UNSTAKE_REPLY_ID: u64 = 8543;
pub const CLAIM_REPLY_ID: u64 = 8546;


impl<T> LocalCwStaking for T where T: AbstractNameService + Execution {}

/// Trait for dispatching *local* staking actions to the appropriate provider
pub trait LocalCwStaking: AbstractNameService + Execution {
    /// resolve the provided dex action on a local dex
    fn resolve_staking_action(
        &self,
        deps: DepsMut,
        action: CwStakingAction,
        exchange: &dyn CwStakingProvider,
        with_reply: bool,
    ) -> Result<SubMsg, StakingError> {
        let (msgs, reply_id) = match action {
            CwStakingAction::Stake { lp_token } => {
                (
                    self.resolve_stake(deps.as_ref(), lp_token, exchange)?,
                    STAKE_REPLY_ID,
                )
            }
            CwStakingAction::Unstake { lp_token } => {
                (
                    self.resolve_unstake(deps.as_ref(), lp_token, exchange)?,
                    UNSTAKE_REPLY_ID,
                )
            }
            CwStakingAction::Claim { lp_token_name } => {
                (
                    self.resolve_claim(deps.as_ref(), lp_token_name, exchange)?,
                    CLAIM_REPLY_ID,
                )
            }
        };
        if with_reply {
            self.executor(deps.as_ref())
                .execute_with_reply(msgs, ReplyOn::Success, reply_id)
        } else {
            self.executor(deps.as_ref()).execute(msgs).map(SubMsg::new)
        }
        .map_err(Into::into)
    }

    fn resolve_stake(
        &self,
        deps: Deps,
        lp_token: LpToken,
        provider: &dyn CwStakingProvider,
    ) -> Result<Vec<CosmosMsg>, StakingError> {
        let ans = self.name_service(deps);

        let staking_address = provider.lp_token_staking_contract_address(
            deps,
            ans.host(),
            lp_token.info.as_str(),
        )?;

        let staking_asset = ans.query(&lp_token)?;

        provider.stake(deps, staking_address, staking_asset)
    }

    fn resolve_unstake(
        &self,
        deps: Deps,
        lp_token: LpToken,
        provider: &dyn CwStakingProvider,
    ) -> Result<Vec<CosmosMsg>, StakingError> {
        let ans = self.name_service(deps);

        let staking_address = provider.lp_token_staking_contract_address(
            deps,
            ans.host(),
            lp_token.info.as_str(),
        )?;

        let staking_asset = ans.query(&lp_token)?;

        provider.unstake(deps, staking_address, staking_asset)
    }

    fn resolve_claim(
        &self,
        deps: Deps,
        lp_token_name: AssetEntry,
        provider: &dyn CwStakingProvider,
    ) -> Result<Vec<CosmosMsg>, StakingError> {
        let ans = self.name_service(deps);

        let staking_address = provider.lp_token_staking_contract_address(
            deps,
            ans.host(),
            lp_token_name.as_str(),
        )?;

        provider.claim(deps, staking_address)
    }
}

// TODO: move these consts
const LP_TOKEN_PROVIDER_SEPARATOR: char = ':';
const LP_TOKEN_ASSET_SEPARATOR: char = '_';

/// Parses the lp token name and returns the assets that make it up
/// The format is: <provider>:<asset1>_<asset2>
/// @todo: move this to abstract
pub fn assets_from_lp_token_name(info: &str) -> Vec<AssetEntry> {
    let words = info.split(LP_TOKEN_PROVIDER_SEPARATOR).collect::<Vec<&str>>();
    let _provider = words[0];
    let assets = words[1].split(LP_TOKEN_ASSET_SEPARATOR).collect::<Vec<&str>>();
    assets.into_iter().map(AssetEntry::from).collect()
}
