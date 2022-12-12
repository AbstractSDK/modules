use abstract_sdk::feature_objects::AnsHost;
use abstract_sdk::os::objects::{AssetEntry, ContractEntry};
use cosmwasm_std::{Addr, CosmosMsg, Decimal, Deps, StdResult, Uint128};
use cw_asset::{Asset, AssetInfo};
use crate::commands::assets_from_lp_token_name;

use crate::error::StakingError;

pub type Return = Uint128;
pub type Spread = Uint128;
pub type Fee = Uint128;
pub type FeeOnInput = bool;

pub trait Identify {
    fn over_ibc(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// Trait that defines the interface for staking providers
pub trait CwStakingProvider: Identify {
    /// Construct the provider entry using the given assets
    fn provider_entry(&self, assets: &mut Vec<&AssetEntry>) -> ContractEntry {
        ContractEntry::construct_dex_entry(self.name(), assets)
    }

    /// Retrieve the staking contract address for the pool with the provided lp token name
    fn lp_token_staking_contract_address(
        &self,
        deps: Deps,
        ans_host: &AnsHost,
        lp_token_name: &str,
    ) -> StdResult<Addr> {
        let lp_token_assets: Vec<AssetEntry> = assets_from_lp_token_name(lp_token_name);
        // Assets by reference
        let mut lp_token_assets = lp_token_assets.iter().map(|a| a).collect();

        let provider_pair = self.provider_entry(&mut lp_token_assets);
        ans_host.query_contract(&deps.querier, &provider_pair)
    }

    /// Stake the provided asset into the staking contract
    ///
    /// * `deps` - the dependencies
    /// * `staking_address` - the address of the staking contract
    /// * `asset` - the asset to stake
    fn stake(
        &self,
        deps: Deps,
        staking_address: Addr,
        asset: Asset
    ) -> Result<Vec<CosmosMsg>, StakingError>;

    /// Stake the provided asset into the staking contract
    ///
    /// * `deps` - the dependencies
    /// * `staking_address` - the address of the staking contract
    /// * `asset` - the asset to stake
    fn unstake(
        &self,
        deps: Deps,
        staking_address: Addr,
        amount: Asset
    ) -> Result<Vec<CosmosMsg>, StakingError>;

    /// Claim rewards on the staking contract
    ///
    /// * `deps` - the dependencies
    /// * `staking_address` - the address of the staking contract
    fn claim(
        &self,
        deps: Deps,
        staking_address: Addr,
    ) -> Result<Vec<CosmosMsg>, StakingError>;
}
