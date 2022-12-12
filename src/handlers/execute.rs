use cosmwasm_std::{Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, to_binary};
use abstract_sdk::os::ibc_client::CallbackInfo;
use abstract_sdk::base::features::AbstractNameService;
use abstract_sdk::{IbcInterface, Resolve};
use abstract_sdk::feature_objects::AnsHost;
use abstract_sdk::os::objects::AnsAsset;
use crate::{contract, provider_resolver, LocalCwStaking, CwStakingProvider};
use crate::contract::{CwStakingExtension, CwStakingResult};
use crate::error::StakingError;
use crate::cw_staking::{CwStakingAction, CwStakingRequestMsg, IBC_STAKING_PROVIDER_ID, LpToken, ProviderName};
use crate::staking_trait::Identify;

const ACTION_RETRIES: u8 = 3;

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    extension: CwStakingExtension,
    msg: CwStakingRequestMsg,
) -> CwStakingResult {
    let CwStakingRequestMsg {
        provider: provider_name,
        action,
    } = msg;
    let exchange = provider_resolver::identify_provider(&provider_name)?;
    // if exchange is on an app-chain, execute the action on the app-chain
    if exchange.over_ibc() {
        handle_ibc_request(&deps, info, &extension, provider_name, &action)
    } else {
        // the action can be executed on the local chain
        handle_local_request(deps, env, info, extension, action, provider_name)
    }
}

/// Handle an extension request that can be executed on the local chain
fn handle_local_request(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    extension: CwStakingExtension,
    action: CwStakingAction,
    exchange: String,
) -> CwStakingResult {
    let exchange = provider_resolver::resolve_local_provider(&exchange)?;
    Ok(
        Response::new()
            .add_submessage(extension.resolve_staking_action(deps, action, exchange, false)?),
    )
}

/// Handle a request that needs to be executed on a remote chain
fn handle_ibc_request(
    deps: &DepsMut,
    info: MessageInfo,
    extension: &CwStakingExtension,
    provider_name: ProviderName,
    action: &CwStakingAction,
) -> CwStakingResult {
    let host_chain = provider_name;
    let ans = extension.name_service(deps.as_ref());
    let ibc_client = extension.ibc_client(deps.as_ref());
    // get the to-be-sent assets from the action
    let coins = resolve_assets_to_transfer(deps.as_ref(), action, ans.host())?;
    // construct the ics20 call(s)
    let ics20_transfer_msg = ibc_client.ics20_transfer(host_chain.clone(), coins)?;
    // construct the action to be called on the host
    let action = abstract_sdk::os::ibc_host::HostAction::App {
        msg: to_binary(&action)?,
    };
    let maybe_contract_info = deps.querier.query_wasm_contract_info(info.sender.clone());
    let callback = if maybe_contract_info.is_err() {
        None
    } else {
        Some(CallbackInfo {
            id: IBC_STAKING_PROVIDER_ID.to_string(),
            receiver: info.sender.into_string(),
        })
    };
    let ibc_action_msg = ibc_client.host_action(host_chain, action, callback, ACTION_RETRIES)?;

    // call both messages on the proxy
    Ok(Response::new().add_messages(vec![ics20_transfer_msg, ibc_action_msg]))
}

/// Resolve the assets to be transferred to the host chain for the given action
fn resolve_assets_to_transfer(
    deps: Deps,
    dex_action: &CwStakingAction,
    ans_host: &AnsHost,
) -> StdResult<Vec<Coin>> {
    match dex_action {
        CwStakingAction::Stake { lp_token, .. } => {
            let resolved: Coin = lp_token.resolve(&deps.querier, ans_host)?.try_into()?;
            Ok(vec![resolved])
        }
        // No assets to transfer
        CwStakingAction::Unstake { .. } => Ok(vec![]),
        // No assets to transfer
        CwStakingAction::Claim { .. } => Ok(vec![]),
        _ => Err(StdError::generic_err("Unsupported action")),
    }
}
