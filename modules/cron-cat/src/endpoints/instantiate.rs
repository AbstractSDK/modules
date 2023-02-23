pub fn instantiate(
    &self,
    ctx: (DepsMut, Env, MessageInfo),
    croncat_manager_addr: String,
    croncat_tasks_addr: String,
    denom: String,
) -> Result<Response, ContractError> {
    let (deps, _, info) = ctx;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    self.owner.save(
        deps.storage,
        &deps.api.addr_canonicalize(&info.sender.as_str())?,
    )?;
    let croncat_manager = deps
        .api
        .addr_canonicalize(&deps.api.addr_validate(&croncat_manager_addr)?.as_str())?;
    let croncat_tasks = deps
        .api
        .addr_canonicalize(&deps.api.addr_validate(&croncat_tasks_addr)?.as_str())?;
    self.croncat_manager.save(deps.storage, &croncat_manager)?;
    self.croncat_tasks.save(deps.storage, &croncat_tasks)?;
    self.denom.save(deps.storage, &denom)?;
    self.action_id.save(deps.storage, &0)?;
    Ok(Response::new())
}
