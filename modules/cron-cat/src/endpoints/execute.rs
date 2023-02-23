use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, ensure, to_binary, CanonicalAddr, CosmosMsg, Deps, DepsMut, Empty, Env, Event,
    MessageInfo, Response, StdResult, SubMsg, Uint128, WasmMsg,
};
use croncat_sdk_manager::msg::ManagerExecuteMsg as CCManagerExecMsg;
use croncat_sdk_tasks::{
    msg::TasksExecuteMsg as CCTaskExecMsg,
    types::{Action, TaskRequest},
};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


pub fn handle_request(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    api: CronCatApp,
    msg: CronCatExecuteMsg,
) -> CronCatResult {
    let executor = api.executor(deps.as_ref());
    let msg = match msg {
        TendermintStakingExecuteMsg::Delegate { validator, amount } => {
            executor.execute(vec![delegate_to(&deps.querier, &validator, amount.u128())?])
        }
        TendermintStakingExecuteMsg::UndelegateFrom { validator, amount } => {
            let undelegate_msg = match amount {
                Some(amount) => undelegate_from(&deps.querier, &validator, amount.u128())?,
                None => undelegate_all_from(&deps.querier, api.target()?, &validator)?,
            };
            executor.execute(vec![undelegate_msg])
        }
        TendermintStakingExecuteMsg::UndelegateAll {} => {
            executor.execute(undelegate_all(&deps.querier, api.target()?)?)
        }

        TendermintStakingExecuteMsg::Redelegate {
            source_validator,
            destination_validator,
            amount,
        } => {
            let redelegate_msg = match amount {
                Some(amount) => redelegate(
                    &deps.querier,
                    &source_validator,
                    &destination_validator,
                    amount.u128(),
                )?,
                None => redelegate_all(
                    &deps.querier,
                    &source_validator,
                    &destination_validator,
                    api.target()?,
                )?,
            };
            executor.execute(vec![redelegate_msg])
        }
        TendermintStakingExecuteMsg::SetWithdrawAddress {
            new_withdraw_address,
        } => executor.execute(vec![update_withdraw_address(
            deps.api,
            &new_withdraw_address,
        )?]),
        TendermintStakingExecuteMsg::WithdrawDelegatorReward { validator } => {
            executor.execute(vec![withdraw_rewards(&validator)])
        }
        TendermintStakingExecuteMsg::WithdrawAllRewards {} => {
            executor.execute(withdraw_all_rewards(&deps.querier, api.target()?)?)
        }
    }?;
    Ok(Response::new().add_message(msg))
}



    pub fn execute(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        action_id: u64,
    ) -> Result<Response, ContractError> {
        let (deps, _, info) = ctx;
        if info.sender
            != deps
                .api
                .addr_humanize(&self.croncat_manager.load(deps.storage)?)?
        {
            Err(ContractError::Unauthorized)
        } else {
            let taskx = self.actions.load(deps.storage, action_id)?;
            let owner = deps
                .api
                .addr_humanize(&self.owner.load(deps.storage)?)?
                .into_string();
            let msg = CosmosMsg::<_>::Wasm(WasmMsg::Execute {
                contract_addr: owner.clone(),
                msg: to_binary(&ProxyExecuteMsg::PluginExecute { msgs: taskx.0 })?,
                funds: vec![],
            });
            let event = Event::new("vectis.cronkitty.v1.MsgExecute").add_attribute("Proxy", owner);
            Ok(Response::new().add_event(event).add_message(msg))
        }
    }

    fn create_task(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        mut task: TaskRequest,
    ) -> Result<Response, ContractError> {
        let (deps, env, info) = ctx;

        // only the owner (proxy) can create task
        if info.sender != deps.api.addr_humanize(&self.owner.load(deps.storage)?)? {
            Err(ContractError::Unauthorized)
        } else {
            // The id for croncat to call the actions in this task
            let id = self.action_id.load(deps.storage)?;
            self.actions.save(
                deps.storage,
                id,
                &(task.actions.iter().cloned().map(|a| a.msg).collect(), None),
            )?;

            // make sure forward all gas
            let gas_limit = task.actions.iter().try_fold(0u64, |acc, a| {
                acc.checked_add(a.gas_limit.unwrap_or(0))
                    .ok_or(ContractError::Overflow)
            })?;
            let denom = self.denom.load(deps.storage)?;
            ensure!(
                info.funds
                    .iter()
                    .find(|c| c.denom == denom)
                    .unwrap_or(&coin(0, denom))
                    .amount
                    >= Uint128::from(gas_limit),
                ContractError::NotEnoughFundsForGas
            );

            let gas_limit = if gas_limit == 0 {
                None
            } else {
                Some(gas_limit)
            };

            let action = Action {
                msg: CosmosMsg::<Empty>::Wasm(WasmMsg::Execute {
                    contract_addr: env.contract.address.to_string(),
                    msg: to_binary(&ExecMsg::Execute { action_id: id })?,
                    funds: vec![],
                }),
                gas_limit,
            };

            // We forward all the other params (so we can contribute / use to frontend code)
            // The Action called is to call this plugin at the given intervals
            task.actions = vec![action];
            task.cw20 = None;

            let msg = SubMsg::reply_on_success(
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: deps
                        .api
                        .addr_humanize(&self.croncat_tasks.load(deps.storage)?)?
                        .to_string(),
                    msg: to_binary(&CCTaskExecMsg::CreateTask {
                        task: Box::new(task),
                    })?,
                    // TODO: https://github.com/CronCats/cw-croncat/issues/204
                    funds: info.funds,
                }),
                id,
            );

            Ok(Response::new().add_submessage(msg))
        }
    }

    pub fn remove_task(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        task_id: u64,
    ) -> Result<Response, ContractError> {
        let (deps, _env, info) = ctx;

        // only the owner (proxy) can create task
        if info.sender != deps.api.addr_humanize(&self.owner.load(deps.storage)?)? {
            Err(ContractError::Unauthorized)
        } else {
            // call croncat to remove task
            if let (_, Some(task_hash)) = self.actions.load(deps.storage, task_id)? {
                let msg = SubMsg::reply_on_success(
                    CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: deps
                            .api
                            .addr_humanize(&self.croncat_tasks.load(deps.storage)?)?
                            .to_string(),
                        msg: to_binary(&CCTaskExecMsg::RemoveTask { task_hash })?,
                        funds: vec![],
                    }),
                    task_id,
                );
                Ok(Response::new().add_submessage(msg))
            } else {
                Err(ContractError::TaskHashNotFound)
            }
        }
    }

    pub fn refill_task(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        task_id: u64,
    ) -> Result<Response, ContractError> {
        let (deps, _env, info) = ctx;

        if info.funds.is_empty() {
            return Err(ContractError::EmptyFunds);
        }

        // only the owner (proxy) can create task
        if info.sender != deps.api.addr_humanize(&self.owner.load(deps.storage)?)? {
            Err(ContractError::Unauthorized)
        } else {
            // call croncat to remove task
            if let (_, Some(task_hash)) = self.actions.load(deps.storage, task_id)? {
                let msg = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: deps
                        .api
                        .addr_humanize(&self.croncat_manager.load(deps.storage)?)?
                        .to_string(),
                    msg: to_binary(&CCManagerExecMsg::RefillTaskBalance { task_hash })?,
                    funds: info.funds,
                });
                Ok(Response::new().add_message(msg))
            } else {
                Err(ContractError::TaskHashNotFound)
            }
        }
    }