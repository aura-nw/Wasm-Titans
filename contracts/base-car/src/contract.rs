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
        ExecuteMsg::Play { turns_to_play } => execute::execute_play(deps, env, info, turns_to_play),
        ExecuteMsg::BuyShell { amount } => execute::execute_buy_shell(deps, env, info, amount),
        ExecuteMsg::BuyAccelerate { amount } => {
            execute::execute_buy_accelerate(deps, env, info, amount)
        }
        ExecuteMsg::BuyBanana {} => todo!(),
        ExecuteMsg::BuyShield { amount } => execute::execute_buy_shield(deps, env, info, amount),
        ExecuteMsg::BuySuperShell { amount } => {
            execute::execute_buy_super_shell(deps, env, info, amount)
        }
        ExecuteMsg::Reset {} => todo!(),
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

    use crate::{
        helpers::{
            get_accel_cost, get_all_car_data_and_find_car, get_banana_cost,
            get_bananas_sorted_by_y, get_shell_cost, get_shield_cost, get_super_shell_cost,
        },
        state::{ActionType, CarData, GameState, GAME_STATE},
        ContractError,
    };

    pub fn execute_reset(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let game_state = GameState::default();
        GAME_STATE.save(deps.storage, &game_state)?;
        Ok(Response::new().add_attribute("action", "execute_reset"))
    }

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
        turns_to_play: u64,
    ) -> Result<Response, ContractError> {
        let mut i = turns_to_play;

        loop {
            if i == 0 {
                break;
            }

            let state = GAME_STATE.load(deps.storage)?;
            let all_cars = state.all_cars.clone();
            let current_turn = state.turns;

            let current_turn_car =
                all_cars[(current_turn % state.config.num_players) as usize].clone();

            let (all_car_data, your_car_index) =
                get_all_car_data_and_find_car(&state, current_turn_car);

            // TODO: Pack msg and send to car contract for running their turn
            // add_message || add_submessage

            let bananas = get_bananas_sorted_by_y(&state);

            for i in 0..state.config.num_players {
                let car_addr = all_cars[i as usize].clone();
                let mut car_data = state.map_addr_car.get(&car_addr).unwrap().to_owned();
                if car_data.shield > 0 {
                    car_data.shield -= 1;
                }

                let len = bananas.len();
                let car_position = car_data.y;
                let mut car_target_position = car_position + car_data.speed;

                for banana_idx in 0..len {
                    let banana_pos = bananas[banana_idx];

                    if car_position >= banana_pos {
                        // Stop at the banana
                        car_target_position = banana_pos
                    }
                }
            }

            i -= 1;
        }

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
        let mut state = GAME_STATE.load(deps.storage)?;

        let cost = get_banana_cost(&state);

        let mut sender_car = state.map_addr_car.get(&info.sender).unwrap().to_owned();

        if state.bananas.len() > 0 && state.bananas[state.bananas.len() - 1] == sender_car.y {
            return Ok(Response::new()
                .add_attribute("cost", "0")
                .add_attribute("turns", state.turns.clone().to_string())
                .add_attribute("action", "buy_banana"));
        }

        sender_car.balance -= cost;

        let y = sender_car.y;

        state.bananas.push(y);
        (*state.action_sold.get_mut(&ActionType::Banana).unwrap()) += 1;

        Ok(Response::new()
            .add_attribute("turns", state.turns.clone().to_string())
            .add_attribute("sender_car", info.sender.to_string())
            .add_attribute("cost", cost.to_string())
            .add_attribute("y", y.to_string())
            .add_attribute("action", "buy_banana"))
    }

    pub fn execute_buy_shield(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: u64,
    ) -> Result<Response, ContractError> {
        if amount == 0 {
            return Err(ContractError::ZeroAmount);
        }

        let mut state = GAME_STATE.load(deps.storage)?;
        let mut sender_car = state.map_addr_car.get(&info.sender).unwrap().to_owned();
        let cost = get_shield_cost(&state, amount);

        sender_car.balance -= cost;

        sender_car.shield += 1 + amount;

        (*state.action_sold.get_mut(&ActionType::Shield).unwrap()) += amount;

        Ok(Response::new()
            .add_attribute("sender_car", info.sender.to_string())
            .add_attribute("amount", amount.to_string())
            .add_attribute("turns", state.turns.clone().to_string())
            .add_attribute("cost", cost.to_string())
            .add_attribute("action", "buy_shield"))
    }

    pub fn execute_buy_super_shell(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: u64,
    ) -> Result<Response, ContractError> {
        if amount == 0 {
            return Err(ContractError::ZeroAmount);
        }

        let mut state = GAME_STATE.load(deps.storage)?;
        let mut sender_car = state.map_addr_car.get(&info.sender).unwrap().to_owned();
        let cost = get_super_shell_cost(&state, amount);

        sender_car.balance -= cost;

        let y = sender_car.y;

        (*state.action_sold.get_mut(&ActionType::SuperShell).unwrap()) += amount;

        let all_cars = state.all_cars.clone();
        for i in 0..state.config.num_players {
            let next_car = state
                .map_addr_car
                .get(&all_cars[i as usize])
                .unwrap()
                .to_owned();
            if next_car.y <= y {
                continue;
            }

            if next_car.speed > state.config.post_sell_speed {
                state.map_addr_car.get_mut(&next_car.addr).unwrap().speed =
                    state.config.post_sell_speed;
                return Ok(Response::new()
                    .add_attribute("cost", cost.to_string())
                    .add_attribute("turns", state.turns.clone().to_string())
                    .add_attribute("action", "shelled"));
            }
        }

        Ok(Response::new()
            .add_attribute("turns", state.turns.clone().to_string())
            .add_attribute("action", "buy_super_shell"))
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
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Empty, OwnedDeps,
    };

    use crate::{contract::instantiate, msg::InstantiateMsg};

    #[test]
    fn test_instantiate_work() {
        let mut deps = mock_dependencies();

        let owner_str = "owner";

        let msg = InstantiateMsg {
            owner: owner_str.to_owned(),
        };

        let info = mock_info("sender", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        println!("len attributes = {:?}", res.attributes.len());
        assert!(res.attributes.len() != 0);
    }

    fn instantiate_deps() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            owner: "owner".to_owned(),
        };

        let info = mock_info("sender", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        deps
    }

    #[test]
    fn test_reset() {}

    #[test]
    fn test_register() {}

    #[test]
    fn test_buy_accel() {}

    #[test]
    fn test_buy_shell() {}

    #[test]
    fn test_buy_ss() {}

    #[test]
    fn test_buy_shield() {}

    #[test]
    fn test_buy_banana() {}

    #[test]
    fn test_play() {}
}
