#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{GameState, GAME_STATE, OWNER};

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
        ExecuteMsg::BuyShell { amount } => execute::execute_buy_shell(deps, env, info, amount),
        ExecuteMsg::BuyAccelerate { amount } => {
            execute::execute_buy_accelerate(deps, env, info, amount)
        }
        ExecuteMsg::BuyBanana {} => todo!(),
        ExecuteMsg::BuyShield { amount } => execute::execute_buy_shield(deps, env, info, amount),
        ExecuteMsg::BuySuperShell { amount } => {
            execute::execute_buy_super_shell(deps, env, info, amount)
        }
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

    use crate::{
        helpers::{get_accel_cost, get_bananas_sorted_by_y, get_shell_cost},
        state::{ActionType, CarData, GAME_STATE},
        ContractError,
    };

    pub fn execute_register(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        car_addr: Addr,
    ) -> Result<Response, ContractError> {
        let mut game_state = GAME_STATE.load(deps.storage)?;

        if game_state.total_cars() == game_state.config.num_players {
            return Err(ContractError::LimitPlayers {});
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
        amount: u64,
    ) -> Result<Response, ContractError> {
        if amount == 0 {
            return Err(ContractError::ZeroAmount);
        }

        let mut state = GAME_STATE.load(deps.storage)?;
        let sender = info.sender;

        let cost = get_shell_cost(&state, amount);

        let mut car_data = state.map_addr_car.get(&sender).unwrap().to_owned();
        car_data.balance -= cost;

        let y = car_data.y;

        (*state.action_sold.get_mut(&ActionType::Shell).unwrap()) += amount;

        let all_cars = state.all_cars.clone();

        let mut closest_car = CarData::empty();
        let mut dis_from_closest_car = u64::MAX;

        for i in 0..state.config.num_players {
            let next_car = state
                .map_addr_car
                .get(&all_cars[i as usize])
                .unwrap()
                .to_owned();

            if next_car.y <= y {
                continue;
            }

            let dis_from_next_car = next_car.y - y;

            if dis_from_next_car < dis_from_closest_car {
                closest_car = next_car;
                dis_from_closest_car = dis_from_next_car
            }
        }

        let len_bananas = state.bananas.len();
        for i in 0..len_bananas {
            if state.bananas[i] <= y {
                continue;
            }

            if dis_from_closest_car != u64::MAX && state.bananas[i] > y + dis_from_closest_car {
                break;
            }

            state.bananas[i] = state.bananas[len_bananas - 1];
            state.bananas.pop();

            let sorted_bananas = get_bananas_sorted_by_y(&state);
            state.bananas = sorted_bananas;
            closest_car = CarData::empty();
            break;
        }

        if closest_car.addr.clone().into_string() != "" {
            if state.map_addr_car.get(&closest_car.addr).unwrap().shield == 0
                && state.map_addr_car.get(&closest_car.addr).unwrap().speed
                    > state.config.post_sell_speed
            {
                state.map_addr_car.get_mut(&closest_car.addr).unwrap().speed =
                    state.config.post_sell_speed;

                return Ok(Response::new()
                    .add_attribute("turns", state.turns.clone().to_string())
                    .add_attribute("smoker", "")
                    .add_attribute("smoked", "")
                    .add_attribute("amount", amount.to_string())
                    .add_attribute("action", "shelled"));
            }
        }

        Ok(Response::new().add_attribute("action", "buy_shell"))
    }

    pub fn execute_buy_accelerate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: u64,
    ) -> Result<Response, ContractError> {
        let mut state = GAME_STATE.load(deps.storage)?;

        let cost = get_accel_cost(&state, amount);

        let sender = info.sender;
        let mut sender_car = state.map_addr_car.get(&sender).unwrap().to_owned();
        sender_car.balance -= cost;

        sender_car.speed += amount;

        (*state.action_sold.get_mut(&ActionType::Accelerate).unwrap()) += amount;

        Ok(Response::new()
            .add_attribute("turns", state.turns.clone().to_string())
            .add_attribute("amount", amount.to_string())
            .add_attribute("cost", cost.to_string())
            .add_attribute("action", "buy_accelerate"))
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
        amount: u64,
    ) -> Result<Response, ContractError> {
        Ok(Response::new().add_attribute("action", "buy_shield"))
    }

    pub fn execute_buy_super_shell(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: u64,
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
