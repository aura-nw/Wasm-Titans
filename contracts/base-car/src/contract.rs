#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{OWNER, GAME_STATE, GameState};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:base-car";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let owner = info.sender.clone();
    OWNER.save(deps.storage, &owner.to_string())?;

    let game_state = GameState::default();
    GAME_STATE.save(deps.storage, &game_state)?;

    Ok(Response::new()
        .add_attribute("owner", owner.to_string())
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
        ExecuteMsg::Register { car_addr } => execute::execute_register(deps, env, info, car_addr),
        ExecuteMsg::Play {} => todo!(),
        ExecuteMsg::BuyShell {} => todo!(),
        ExecuteMsg::BuyAccelerate {} => todo!(),
        ExecuteMsg::BuyBanana {} => todo!(),
        ExecuteMsg::BuyShield {} => todo!(),
        ExecuteMsg::BuySuperShell {} => todo!(),
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

    use crate::{ContractError, state::GAME_STATE};

    pub fn execute_register(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        car_addr: Addr,
    ) -> Result<Response, ContractError> {
        let mut game_state = GAME_STATE.load(deps.storage)?;

        if game_state.total_cars() == game_state.config.num_players {
            return Err(ContractError::LimitPlayers{});
        }
        
        game_state.register(car_addr.clone());

        GAME_STATE.save(deps.storage, &game_state)?;

        Ok(Response::new()
            .add_attribute("car_address", car_addr.clone().to_string())
            .add_attribute("action", "register"))
    }

    pub fn execute_play(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_attribute("action", "play"))
    }

    pub fn execute_buy_shell(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_attribute("action", "buy_shell"))
    }

    pub fn execute_buy_accelerate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_attribute("action", "buy_accelerate"))
    }

    pub fn execute_buy_banana(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_attribute("action", "buy_banana"))
    }

    pub fn execute_buy_shield(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_attribute("action", "buy_shield"))
    }

    pub fn execute_buy_super_shell(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_attribute("action", "buy_super_shell"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAllCarData {} => todo!(),
        QueryMsg::GetAllBananas {} => todo!(),
        QueryMsg::GetAccelerateCost {} => todo!(),
        QueryMsg::GetShellCost {} => todo!(),
        QueryMsg::GetSuperShellCost {} => todo!(),
        QueryMsg::GetBananaCost {} => todo!(),
        QueryMsg::GetShieldCost => todo!(),
        QueryMsg::GetOwner {} => todo!(),
    }
}

#[cfg(test)]
mod tests {}
