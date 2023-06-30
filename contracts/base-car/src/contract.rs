#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ActionType, CarData, GameState, ACTION_SOLD, ALL_CAR_DATA, GAME_STATE, OWNER};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let owner = info.sender.clone();
    OWNER.save(deps.storage, &owner.to_string())?;

    ACTION_SOLD.save(deps.storage, &ActionType::Accelerate.to_string(), &0)?;
    ACTION_SOLD.save(deps.storage, &ActionType::Shell.to_string(), &0)?;
    ACTION_SOLD.save(deps.storage, &ActionType::SuperShell.to_string(), &0)?;
    ACTION_SOLD.save(deps.storage, &ActionType::Banana.to_string(), &0)?;
    ACTION_SOLD.save(deps.storage, &ActionType::Shield.to_string(), &0)?;

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
        ExecuteMsg::Register { car_addrs } => execute::execute_register(deps, env, info, car_addrs),
        ExecuteMsg::Play { turns_to_play } => execute::execute_play(deps, env, info, turns_to_play),
        ExecuteMsg::BuyShell { amount } => execute::execute_buy_shell(deps, env, info, amount),
        ExecuteMsg::BuyAccelerate { amount } => {
            execute::execute_buy_accelerate(deps, env, info, amount)
        }
        ExecuteMsg::BuyBanana {} => execute::execute_buy_banana(deps, env, info),
        ExecuteMsg::BuyShield { amount } => execute::execute_buy_shield(deps, env, info, amount),
        ExecuteMsg::BuySuperShell { amount } => {
            execute::execute_buy_super_shell(deps, env, info, amount)
        }
        ExecuteMsg::Reset {} => execute::execute_reset(deps, env, info),
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};

    use crate::{
        helpers::{
            get_accel_cost, get_banana_cost, get_bananas_sorted_by_y, get_shell_cost,
            get_shield_cost, get_super_shell_cost,
        },
        state::{
            ActionType, CarData, GameState, State, ACTION_SOLD, ALL_CAR_DATA, GAME_STATE, OWNER,
        },
        ContractError,
    };

    use super::get_all_car_data_and_find_car;

    pub fn execute_reset(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;

        if info.sender.to_string() != owner {
            return Err(ContractError::Unauthorized {});
        }

        let game_state = GameState::default();
        GAME_STATE.save(deps.storage, &game_state)?;
        Ok(Response::new().add_attribute("action", "execute_reset"))
    }

    pub fn execute_register(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        car_addrs: Vec<Addr>,
    ) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;

        if info.sender.to_string() != owner {
            return Err(ContractError::Unauthorized {});
        }

        let mut game_state = GAME_STATE.load(deps.storage)?;

        if game_state.total_cars() == game_state.config.num_players {
            return Err(ContractError::LimitPlayers {});
        }

        game_state.register(car_addrs.clone());
        game_state.state = State::Active;

        GAME_STATE.save(deps.storage, &game_state)?;

        for car_addr in car_addrs.clone() {
            ALL_CAR_DATA.save(deps.storage, car_addr.clone(), &CarData::at_start(car_addr))?;
        }

        Ok(Response::new()
            .add_attribute(
                "cars",
                car_addrs
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("|"),
            )
            .add_attribute("action", "register"))
    }

    pub fn execute_play(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        turns_to_play: u64,
    ) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;

        if info.sender.to_string() != owner {
            return Err(ContractError::Unauthorized {});
        }

        let mut i = turns_to_play;

        loop {
            if i == 0 {
                break;
            }

            let state = GAME_STATE.load(deps.storage)?;

            if !state.can_play() {
                return Err(ContractError::NotEnoughPlayers);
            }

            let all_cars = state.all_cars.clone();
            let current_turn = state.turns;

            let current_turn_car =
                all_cars[(current_turn % state.config.num_players) as usize].clone();

            let (all_car_data, your_car_index) =
                get_all_car_data_and_find_car(deps.as_ref(), &state, current_turn_car);

            // TODO: Pack msg and send to car contract for running their turn
            // add_message || add_submessage

            let bananas = get_bananas_sorted_by_y(&state);

            for i in 0..state.config.num_players {
                let car_addr = all_cars[i as usize].clone();
                let mut car_data = ALL_CAR_DATA.load(deps.storage, car_addr)?;
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
        _env: Env,
        info: MessageInfo,
        amount: u64,
    ) -> Result<Response, ContractError> {
        if amount == 0 {
            return Err(ContractError::ZeroAmount);
        }

        let mut state = GAME_STATE.load(deps.storage)?;
        let sender = info.sender;

        let sold = ACTION_SOLD.load(deps.storage, &ActionType::Shell.to_string())?;
        let cost = get_shell_cost(&state, amount, sold.clone());

        let sold_updated = sold + amount;

        ACTION_SOLD.save(deps.storage, &ActionType::Shell.to_string(), &sold_updated)?;

        let mut car_data = ALL_CAR_DATA.load(deps.storage, sender.clone())?;
        car_data.balance -= cost;

        let y = car_data.y;

        let all_cars = state.all_cars.clone();

        // Used to determine who to shell.
        let mut closest_car = CarData::empty();
        let mut dis_from_closest_car = u64::MAX;

        for i in 0..state.config.num_players {
            let next_car = ALL_CAR_DATA.load(deps.storage, all_cars[i as usize].clone())?;
            
            // If the car is behind or on us, skip it
            if next_car.y <= y {
                continue;
            }

            // Measure the distance from the car to us
            let dis_from_next_car = next_car.y - y;
            
            // If this car is closer than all other cars we've
            // looked at so far, we'll make it than closest one.
            if dis_from_next_car < dis_from_closest_car {
                closest_car = next_car;
                dis_from_closest_car = dis_from_next_car
            }
        }

        // Check for banana collisions
        for i in 0..state.bananas.len() {
            // Skip bananas that are behind or on us
            if state.bananas[i] <= y {
                continue;
            }

            // Check if the closest car is closer than the closest banana
            // If a banana is on top of the colest car, the banana is hit
            if dis_from_closest_car != u64::MAX 
                && state.bananas[i] > y + dis_from_closest_car {
                break;
            }

            // Remove the banana by swapping it with the last and decreasing the size
            state.bananas[i] = state.bananas[state.bananas.len() - 1].clone();
            state.bananas.pop();

            // Sort the bananas
            let sorted_bananas = get_bananas_sorted_by_y(&state);
            state.bananas = sorted_bananas;

            // Banana was closer or at the same position as the closestCar
            closest_car = CarData::empty();
            break;
        }

        // If there is a closest car, shell it.
        if closest_car.addr.clone().into_string() != "" {
            if closest_car.shield == 0 && closest_car.speed > state.config.post_sell_speed {
                closest_car.speed = state.config.post_sell_speed;
                return Ok(Response::new()
                    .add_attribute("turns", state.turns.clone().to_string())
                    .add_attribute("sender", sender.clone().to_string())
                    .add_attribute("shelled", closest_car.clone().addr.to_string())
                    .add_attribute("amount", amount.to_string())
                    .add_attribute("action", "shelled"));
            }
        }
        
        // No car has shelled
        Ok(Response::new()
            .add_attribute("sender", sender.to_string())
            .add_attribute("turns", state.turns.clone().to_string())
            .add_attribute("action", "buy_shell"))
    }

    pub fn execute_buy_accelerate(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        amount: u64,
    ) -> Result<Response, ContractError> {
        let state = GAME_STATE.load(deps.storage)?;
        let sold = ACTION_SOLD.load(deps.storage, &ActionType::Accelerate.to_string())?;

        // Get the cost of the acceleration
        let cost = get_accel_cost(&state, amount, sold.clone());

        // Increase amount of acceleration sold
        let sold_updated = sold + amount;
        ACTION_SOLD.save(deps.storage, &ActionType::Shell.to_string(), &sold_updated)?;
        
        let sender = info.sender;

        let mut sender_car = ALL_CAR_DATA.load(deps.storage, sender.clone())?;
        sender_car.balance -= cost;
        sender_car.speed += amount;

        Ok(Response::new()
            .add_attribute("turns", state.turns.to_string())
            .add_attribute("sender_car", sender.to_string())
            .add_attribute("amount", amount.to_string())
            .add_attribute("cost", cost.to_string())
            .add_attribute("action", "buy_accelerate"))
    }

    pub fn execute_buy_banana(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let mut state = GAME_STATE.load(deps.storage)?;
        let mut sender_car = ALL_CAR_DATA.load(deps.storage, info.sender.clone())?;

        if state.bananas.len() > 0 && state.bananas[state.bananas.len() - 1] == sender_car.y {
            return Ok(Response::new()
                .add_attribute("turns", state.turns.clone().to_string())
                .add_attribute("sender_car", info.sender.clone().to_string())
                .add_attribute("action", "buy_banana"));
        }

        let sold = ACTION_SOLD.load(deps.storage, &ActionType::Banana.to_string())?;
        let cost = get_banana_cost(&state, sold.clone());

        let sold_updated = sold + 1;

        ACTION_SOLD.save(deps.storage, &ActionType::Banana.to_string(), &sold_updated)?;

        sender_car.balance -= cost;

        let y = sender_car.y;

        state.bananas.push(y);

        Ok(Response::new()
            .add_attribute("turns", state.turns.clone().to_string())
            .add_attribute("sender_car", info.sender.clone().to_string())
            .add_attribute("cost", cost.to_string())
            .add_attribute("y", y.to_string())
            .add_attribute("action", "buy_banana"))
    }

    pub fn execute_buy_shield(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        amount: u64,
    ) -> Result<Response, ContractError> {
        if amount == 0 {
            return Err(ContractError::ZeroAmount);
        }

        let state = GAME_STATE.load(deps.storage)?;
        let mut sender_car = ALL_CAR_DATA.load(deps.storage, info.sender.clone())?;
        let sold = ACTION_SOLD.load(deps.storage, &ActionType::Shield.to_string())?;
        let cost = get_shield_cost(&state, amount, sold.clone());

        let sold_updated = sold + amount;

        ACTION_SOLD.save(deps.storage, &ActionType::Shield.to_string(), &sold_updated)?;
        sender_car.balance -= cost;

        sender_car.shield += 1 + amount;

        Ok(Response::new()
            .add_attribute("sender_car", info.sender.clone().to_string())
            .add_attribute("amount", amount.to_string())
            .add_attribute("turns", state.turns.clone().to_string())
            .add_attribute("cost", cost.to_string())
            .add_attribute("action", "buy_shield"))
    }

    pub fn execute_buy_super_shell(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        amount: u64,
    ) -> Result<Response, ContractError> {
        if amount == 0 {
            return Err(ContractError::ZeroAmount);
        }

        let state = GAME_STATE.load(deps.storage)?;
        let mut sender_car = ALL_CAR_DATA.load(deps.storage, info.sender.clone())?;
        let sold = ACTION_SOLD.load(deps.storage, &ActionType::SuperShell.to_string())?;
        let cost = get_super_shell_cost(&state, amount, sold.clone());

        let sold_updated = sold + amount;

        ACTION_SOLD.save(
            deps.storage,
            &ActionType::SuperShell.to_string(),
            &sold_updated,
        )?;
        sender_car.balance -= cost;

        let y = sender_car.y;

        let all_cars = state.all_cars.clone();
        for i in 0..state.config.num_players {
            let mut next_car = ALL_CAR_DATA.load(deps.storage, all_cars[i as usize].clone())?;
            if next_car.y <= y {
                continue;
            }

            if next_car.speed > state.config.post_sell_speed {
                next_car.speed = state.config.post_sell_speed;
                return Ok(Response::new()
                    .add_attribute("cost", cost.to_string())
                    .add_attribute("turns", state.turns.clone().to_string())
                    .add_attribute("action", "shelled"));
            }
        }

        Ok(Response::new()
            .add_attribute("turns", state.turns.clone().to_string())
            .add_attribute("sender_car", info.sender.clone().to_string())
            .add_attribute("action", "buy_super_shell"))
    }
}

pub fn get_cars_sorted_by_y(deps: Deps, state: &GameState) -> Vec<Addr> {
    let mut cars = state.all_cars.clone();

    for i in 0..state.config.num_players {
        for j in (i + 1)..state.config.num_players {
            let car_data_result_j =
                ALL_CAR_DATA.load(deps.storage, state.all_cars[j as usize].clone());
            if car_data_result_j.is_err() {
                return vec![];
            }
            let car_data_j = car_data_result_j.unwrap();

            let car_data_result_i =
                ALL_CAR_DATA.load(deps.storage, state.all_cars[i as usize].clone());
            if car_data_result_i.is_err() {
                return vec![];
            }
            let car_data_i = car_data_result_i.unwrap();

            if car_data_j.y > car_data_i.y {
                let temp = cars[i as usize].clone();
                cars[i as usize] = cars[j as usize].clone();
                cars[j as usize] = temp
            }
        }
    }

    cars
}

pub fn get_all_car_data_and_find_car(
    deps: Deps,
    state: &GameState,
    addr: Addr,
) -> (Vec<CarData>, Option<u64>) {
    let mut results = Vec::new();
    let mut found_car_index = None;

    let sorted_cars = get_cars_sorted_by_y(deps, state);

    for i in 0..(state.config.num_players) {
        let car_addr = sorted_cars[i as usize].clone();

        if car_addr == addr {
            found_car_index = Some(i as u64);
        }
        let car_data_result = ALL_CAR_DATA.load(deps.storage, car_addr);
        if car_data_result.is_err() {
            return (vec![], None);
        }
        let car_data = car_data_result.unwrap();
        results.push(car_data);
    }

    (results, found_car_index)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAllCarData => to_binary(&query::get_all_car_data(deps)?),
        QueryMsg::GetOwner => to_binary(&query::get_owner(deps)?),
        QueryMsg::GetGameState => to_binary(&query::get_game_state(deps)?),
    }
}

pub mod query {
    use std::vec;

    use cosmwasm_std::{Addr, Deps, Order, StdResult};

    use crate::{
        msg::{AllCarDataReponse, GameStateResponse, OwnerResponse},
        state::{CarData, ALL_CAR_DATA, GAME_STATE, OWNER},
    };

    pub fn get_all_car_data(deps: Deps) -> StdResult<AllCarDataReponse> {
        let all: StdResult<Vec<(Addr, CarData)>> = ALL_CAR_DATA
            .range(deps.storage, None, None, Order::Ascending)
            .collect();
        if all.is_err() {
            return Ok(AllCarDataReponse { all_cars: vec![] });
        }
        Ok(AllCarDataReponse {
            all_cars: all.unwrap(),
        })
    }

    pub fn get_owner(deps: Deps) -> StdResult<OwnerResponse> {
        let owner = OWNER.may_load(deps.storage)?;
        Ok(OwnerResponse { owner })
    }

    pub fn get_game_state(deps: Deps) -> StdResult<GameStateResponse> {
        let game_sate = GAME_STATE.load(deps.storage)?;
        Ok(GameStateResponse {
            turns: game_sate.turns,
            config: game_sate.config,
            state: game_sate.state,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Addr, Empty, OwnedDeps,
    };

    use crate::{
        contract::execute,
        contract::instantiate,
        msg::{
            AllCarDataReponse, ExecuteMsg, GameStateResponse, InstantiateMsg, OwnerResponse,
            QueryMsg,
        },
        state::{State, GAME_STATE},
    };

    use super::{get_all_car_data_and_find_car, get_cars_sorted_by_y, query};

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

        let info = mock_info("owner", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        deps
    }

    fn register_deps() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
        let mut deps = instantiate_deps();
        let msg = ExecuteMsg::Register {
            car_addrs: vec![
                Addr::unchecked("car1"),
                Addr::unchecked("car2"),
                Addr::unchecked("car3"),
            ],
        };

        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("owner", &[]),
            msg.clone(),
        );
        assert!(res.is_ok());

        deps
    }

    #[test]
    fn test_reset() {
        let mut deps = instantiate_deps();

        let msg = ExecuteMsg::Reset {};

        let owner = "owner";
        let not_owner = "not_owner";
        let owner_info = mock_info(owner, &vec![]);
        let not_onwer_info = mock_info(not_owner, &vec![]);

        let res = execute(deps.as_mut(), mock_env(), owner_info, msg.clone());
        assert!(res.is_ok());

        let res = execute(deps.as_mut(), mock_env(), not_onwer_info, msg.clone());
        assert!(res.is_err());
    }

    #[test]
    fn test_register() {
        let mut deps = instantiate_deps();

        let msg = ExecuteMsg::Register {
            car_addrs: vec![
                Addr::unchecked("car1"),
                Addr::unchecked("car2"),
                Addr::unchecked("car3"),
            ],
        };

        let owner = "owner";
        let not_owner = "not_owner";

        let owner_info = mock_info(owner, &[]);
        let not_owner_info = mock_info(not_owner, &[]);

        let res = execute(deps.as_mut(), mock_env(), not_owner_info, msg.clone());
        assert!(res.is_err());

        let res = execute(deps.as_mut(), mock_env(), owner_info.clone(), msg.clone());
        assert!(res.is_ok());
    }

    #[test]
    fn test_buy_accel() {
        let mut deps = register_deps();

        let msg = ExecuteMsg::BuyAccelerate { amount: 1 };

        let info = mock_info("car1", &[]);

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        println!("res: {:?}", res);

        assert!(res.is_ok());
    }

    #[test]
    fn test_buy_shell() {
        let mut deps = register_deps();

        let msg = ExecuteMsg::BuyShell { amount: 1 };

        let info = mock_info("car1", &[]);

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res.is_ok());
    }

    #[test]
    fn test_buy_ss() {
        let mut deps = register_deps();

        let msg = ExecuteMsg::BuySuperShell { amount: 1 };

        let info = mock_info("car1", &[]);

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res.is_ok());
    }

    #[test]
    fn test_buy_shield() {
        let mut deps = register_deps();

        let msg = ExecuteMsg::BuyShield { amount: 1 };

        let info = mock_info("car1", &[]);

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res.is_ok());
    }

    #[test]
    fn test_buy_banana() {
        let mut deps = register_deps();

        let msg = ExecuteMsg::BuyBanana {};

        let info = mock_info("car1", &[]);

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res.is_ok());
    }

    #[test]
    fn test_query_owner() {
        let deps = instantiate_deps();

        let msg = QueryMsg::GetOwner;

        let res = query(deps.as_ref(), mock_env(), msg);

        assert!(res.is_ok());

        let OwnerResponse { owner } = from_binary(&res.unwrap()).unwrap();

        println!("res: {:?}", owner.unwrap());

        let deps = register_deps();

        let msg = QueryMsg::GetOwner;

        let res = query(deps.as_ref(), mock_env(), msg);

        assert!(res.is_ok());

        let OwnerResponse { owner } = from_binary(&res.unwrap()).unwrap();

        println!("res: {:?}", owner.unwrap());
    }

    #[test]
    fn test_query_all_car_data() {
        let deps = instantiate_deps();
        let msg = QueryMsg::GetAllCarData;
        let res = query(deps.as_ref(), mock_env(), msg);
        assert!(res.is_ok());
        let AllCarDataReponse { all_cars } = from_binary(&res.unwrap()).unwrap();
        println!("all_cars: {:?}", all_cars);
        assert!(all_cars.len() == 0);

        let deps = register_deps();
        let msg = QueryMsg::GetAllCarData {};
        let res = query(deps.as_ref(), mock_env(), msg);
        assert!(res.is_ok());
        let AllCarDataReponse { all_cars } = from_binary(&res.unwrap()).unwrap();
        println!("all_cars: {:?}", all_cars);
        assert!(all_cars.len() == 3);
    }

    #[test]
    fn test_query_game_state() {
        let deps = instantiate_deps();
        let msg = QueryMsg::GetGameState;
        let res = query(deps.as_ref(), mock_env(), msg);
        assert!(res.is_ok());
        let GameStateResponse {
            turns,
            config,
            state,
        } = from_binary(&res.unwrap()).unwrap();
        println!("turns: {:?}", turns);
        assert!(turns == 0);
        println!("config: {:?}", config);
        println!("state: {:?}", state);
        assert!(state == State::Waiting);

        let deps = register_deps();
        let msg = QueryMsg::GetGameState;
        let res = query(deps.as_ref(), mock_env(), msg);
        assert!(res.is_ok());
        let GameStateResponse {
            turns,
            config,
            state,
        } = from_binary(&res.unwrap()).unwrap();
        println!("turns: {:?}", turns);
        assert!(turns == 0);
        println!("config: {:?}", config);
        println!("state: {:?}", state);
        assert!(state == State::Active);
    }

    #[test]
    fn test_get_cars_sorted_by_y() {
        let deps = register_deps();
        let game_state = GAME_STATE.load(&deps.storage).unwrap();

        let cars = get_cars_sorted_by_y(deps.as_ref(), &game_state);
        assert!(cars.len() == 3);
        println!("cars: {:?}", cars);
    }

    #[test]
    fn test_get_all_car_data_and_find_car() {
        let deps = register_deps();
        let game_state = GAME_STATE.load(&deps.storage).unwrap();

        let (all_car_data, found_car) =
            get_all_car_data_and_find_car(deps.as_ref(), &game_state, Addr::unchecked("car2"));

        println!("all_car_data: {:?}", all_car_data);
        println!("found_car: {:?}", found_car);
    }

    #[test]
    fn test_play() {}
}
