
use cosmwasm_std::{
    Addr, Coin, CosmosMsg, Deps, StdResult, to_binary, WasmMsg,
};

use cw_asset::{Asset, AssetInfo, AssetInfoBase};
use crate::error::StakingError;
use crate::traits::cw_staking_provider::CwStakingProvider;
use crate::traits::identify::Identify;

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
    fn stake(&self, _deps: Deps, _staking_address: Addr, _asset: Asset) -> Result<Vec<CosmosMsg>, StakingError> {
        unimplemented!()
    }

    fn unstake(&self, _deps: Deps, _staking_address: Addr, _amount: Asset) -> Result<Vec<CosmosMsg>, StakingError> {
        unimplemented!()
    }

    fn claim(&self, _deps: Deps, _staking_address: Addr) -> Result<Vec<CosmosMsg>, StakingError> {
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
