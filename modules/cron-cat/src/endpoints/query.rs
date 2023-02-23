
#[msg(query)]
pub fn action_id(&self, ctx: (Deps, Env)) -> StdResult<u64> {
    let (deps, _) = ctx;
    self.action_id.load(deps.storage)
}

// These are the id that stores the actual cosmos messages
#[msg(query)]
pub fn action(&self, ctx: (Deps, Env), action_id: u64) -> StdResult<CronKittyActionResp> {
    let (deps, _) = ctx;
    let (msgs, task_hash) = self.actions.load(deps.storage, action_id)?;
    Ok(CronKittyActionResp { msgs, task_hash })
}
