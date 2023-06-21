#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::BASE_CAR_ADDR;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:car-1";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let base_car_addr = msg.base_car_addr.clone();
    BASE_CAR_ADDR.save(deps.storage, &base_car_addr)?;
    Ok(Response::new()
        .add_attribute("base_car_addr", base_car_addr)
        .add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TakeTurn {} => execute::execute_take_turn(deps, env, info),
    }
}

pub mod execute {
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

    use crate::ContractError;

    pub fn execute_take_turn(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        Ok(Response::new()
            .add_attribute("contract_addr", env.contract.address.clone().to_string())
            .add_attribute("action", "execute_take_turn"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
