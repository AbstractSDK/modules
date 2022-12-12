
use cosmwasm_std::{
    Addr, Coin, CosmosMsg, Deps, StdResult,
    to_binary, WasmMsg,
};
use cw20_junoswap::Denom;
use cw_asset::{Asset, AssetInfo};
use crate::error::StakingError;
use crate::traits::cw_staking_provider::CwStakingProvider;
use crate::traits::identify::Identify;

pub const JUNOSWAP: &str = "junoswap";
// Source https://github.com/wasmswap/wasmswap-contracts
pub struct JunoSwap {}

impl Identify for JunoSwap {
    fn over_ibc(&self) -> bool {
        false
    }
    fn name(&self) -> &'static str {
        JUNOSWAP
    }
}

impl CwStakingProvider for JunoSwap {
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

fn _denom_and_asset_match(denom: &Denom, asset: &AssetInfo) -> Result<bool, StakingError> {
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
