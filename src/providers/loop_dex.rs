use crate::{staking_trait::Identify, error::StakingError, CwStakingProvider};

use cosmwasm_std::{
    to_binary, wasm_execute, Addr, Coin, CosmosMsg, Decimal, Deps, QueryRequest, StdResult,
    Uint128, WasmMsg, WasmQuery,
};
use cw20::Cw20ExecuteMsg;
use cw_asset::{Asset, AssetInfo, AssetInfoBase};
use terraswap::pair::{PoolResponse, SimulationResponse};
pub const LOOP: &str = "loop";
pub struct Loop {}

impl Identify for Loop {
    fn name(&self) -> &'static str {
        LOOP
    }
    fn over_ibc(&self) -> bool {
        false
    }
}

impl CwStakingProvider for Loop {
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

fn cw_asset_to_terraswap(asset: &Asset) -> Result<terraswap::asset::Asset, StakingError> {
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
