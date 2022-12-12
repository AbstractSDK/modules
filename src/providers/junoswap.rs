use crate::{staking_trait::Identify, error::StakingError, CwStakingProvider};

use cosmwasm_std::{
    to_binary, wasm_execute, Addr, Coin, CosmosMsg, Decimal, Deps, Fraction, QueryRequest,
    StdResult, Uint128, WasmMsg, WasmQuery,
};
use cw20_junoswap::{Cw20ExecuteMsg, Denom};
use cw_asset::{Asset, AssetInfo, AssetInfoBase};
use wasmswap::msg::*;
pub const JUNOSWAP: &str = "junoswap";
// Source https://github.com/wasmswap/wasmswap-contracts
pub struct JunoSwap {}

impl Identify for JunoSwap {
    fn name(&self) -> &'static str {
        JUNOSWAP
    }
    fn over_ibc(&self) -> bool {
        false
    }
}

impl CwStakingProvider for JunoSwap {
    fn stake(&self, deps: Deps, staking_address: Addr, asset: Asset) -> Result<Vec<CosmosMsg>, StakingError> {
        unimplemented!()
    }

    fn unstake(&self, deps: Deps, staking_address: Addr, amount: Asset) -> Result<Vec<CosmosMsg>, StakingError> {
        unimplemented!()
    }

    fn claim(&self, deps: Deps, staking_address: Addr) -> Result<Vec<CosmosMsg>, StakingError> {
        unimplemented!()
    }
}

fn denom_and_asset_match(denom: &Denom, asset: &AssetInfo) -> Result<bool, StakingError> {
    match denom {
        Denom::Native(denom_name) => match asset {
            cw_asset::AssetInfoBase::Native(asset_name) => Ok(denom_name == asset_name),
            cw_asset::AssetInfoBase::Cw20(_asset_addr) => Ok(false),
            cw_asset::AssetInfoBase::Cw1155(_, _) => Err(StakingError::Cw1155Unsupported),
            _ => panic!("unsupported asset"),
        },
        Denom::Cw20(denom_addr) => match asset {
            cw_asset::AssetInfoBase::Native(_asset_name) => Ok(false),
            cw_asset::AssetInfoBase::Cw20(asset_addr) => Ok(denom_addr == asset_addr),
            cw_asset::AssetInfoBase::Cw1155(_, _) => Err(StakingError::Cw1155Unsupported),
            _ => panic!("unsupported asset"),
        },
    }
}

fn cw_approve_msgs(assets: &[Asset], spender: &Addr) -> StdResult<Vec<CosmosMsg>> {
    let mut msgs = vec![];
    for asset in assets {
        if let AssetInfo::Cw20(addr) = &asset.info {
            let msg = cw20_junoswap::Cw20ExecuteMsg::IncreaseAllowance {
                spender: spender.to_string(),
                amount: asset.amount,
                expires: None,
            };
            msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: addr.to_string(),
                msg: to_binary(&msg)?,
                funds: vec![],
            }))
        }
    }
    Ok(msgs)
}

fn coins_in_assets(assets: &[Asset]) -> Vec<Coin> {
    let mut coins = vec![];
    for asset in assets {
        if let AssetInfo::Native(denom) = &asset.info {
            coins.push(Coin::new(asset.amount.u128(), denom.clone()));
        }
    }
    coins
}
