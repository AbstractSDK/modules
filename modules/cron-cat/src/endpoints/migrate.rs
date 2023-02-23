
#[msg(migrate)]
fn migrate(&self, _ctx: (DepsMut, Env)) -> Result<Response, ContractError> {
    // Not used but required for impl for multitest
    Ok(Response::default())
}