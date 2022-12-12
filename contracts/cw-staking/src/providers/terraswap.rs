use crate::{
    contract::{CwStakingExtension, CwStakingResult},
    error::StakingError,
    CwStakingProvider,
};

use abstract_sdk::OsExecute;
use cosmwasm_std::{
    to_binary, wasm_execute, Addr, Coin, CosmosMsg, Decimal, Deps, QueryRequest, StdResult,
    Uint128, WasmMsg, WasmQuery,
};
use cw20::Cw20ExecuteMsg;
use cw_asset::{Asset, AssetInfo, AssetInfoBase};
use terraswap::pair::{PoolResponse, SimulationResponse};
use crate::traits::identify::Identify;

pub const TERRASWAP: &str = "terraswap";
pub struct Terraswap {}

impl Identify for Terraswap {
    fn over_ibc(&self) -> bool {
        false
    }
    fn name(&self) -> &'static str {
        TERRASWAP
    }
}

impl CwStakingProvider for Terraswap {
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

fn _cw_asset_to_terraswap(asset: &Asset) -> Result<terraswap::asset::Asset, StakingError> {
    match &asset.info {
        AssetInfoBase::Native(denom) => Ok(terraswap::asset::Asset {
            amount: asset.amount,
            info: terraswap::asset::AssetInfo::NativeToken {
                denom: denom.clone(),
            },
        }),
        AssetInfoBase::Cw20(contract_addr) => Ok(terraswap::asset::Asset {
            amount: asset.amount,
            info: terraswap::asset::AssetInfo::Token {
                contract_addr: contract_addr.to_string(),
            },
        }),
        _ => Err(StakingError::Cw1155Unsupported {}),
    }
}

fn _cw_approve_msgs(assets: &[Asset], spender: &Addr) -> StdResult<Vec<CosmosMsg>> {
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

fn _coins_in_assets(assets: &[Asset]) -> Vec<Coin> {
    let mut coins = vec![];
    for asset in assets {
        if let AssetInfo::Native(denom) = &asset.info {
            coins.push(Coin::new(asset.amount.u128(), denom.clone()));
        }
    }
    coins
}
