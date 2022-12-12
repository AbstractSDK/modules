use abstract_sdk::feature_objects::AnsHost;
use abstract_sdk::os::objects::{AssetEntry, ContractEntry};
use cosmwasm_std::{Addr, CosmosMsg, Deps, StdResult};
use cw_asset::Asset;

use crate::error::StakingError;
use crate::traits::identify::Identify;

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
        let mut lp_token_assets = lp_token_assets.iter().collect();

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


// TODO: move these consts
const LP_TOKEN_PROVIDER_SEPARATOR: char = ':';
const LP_TOKEN_ASSET_SEPARATOR: char = '_';


/// Parses the lp token name and returns the assets that make it up
/// The format is: <provider>:<asset1>_<asset2>
/// @todo: move this to abstract
pub fn assets_from_lp_token_name(info: &str) -> Vec<AssetEntry> {
    let words = info.split(LP_TOKEN_PROVIDER_SEPARATOR).collect::<Vec<&str>>();
    let _provider = words[0];
    words[1].split(LP_TOKEN_ASSET_SEPARATOR).map(AssetEntry::from).collect()
}
