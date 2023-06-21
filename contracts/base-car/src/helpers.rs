use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

use crate::{
    msg::ExecuteMsg,
    state::{ActionType, CarData, GameState},
};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

pub fn get_accel_cost(state: &GameState, amount: u64) -> u64 {
    let mut sum = 0 as u64;
    for i in 0..amount {
        sum += compute_action_price(
            state.config.accel_target_price,
            state.config.accel_per_turn_decrease,
            state.turns,
            state.action_sold.get(&ActionType::Accelerate).unwrap() + i,
            state.config.accel_sell_per_turn,
        );
    }
    sum
}

pub fn get_shell_cost(state: &GameState, amount: u64) -> u64 {
    let mut sum = 0 as u64;

    for i in 0..amount {
        sum += compute_action_price(
            state.config.shell_target_price,
            state.config.shell_per_turn_decrease,
            state.turns,
            state.action_sold.get(&ActionType::Shell).unwrap() + i,
            state.config.shell_sell_per_turn,
        );
    }
    sum
}

pub fn get_super_shell_cost(state: &GameState, amount: u64) -> u64 {
    let mut sum = 0 as u64;

    for i in 0..amount {
        sum += compute_action_price(
            state.config.ss_target_price,
            state.config.ss_per_turn_decrease,
            state.turns,
            state.action_sold.get(&ActionType::SuperShell).unwrap() + i,
            state.config.ss_sell_per_turn,
        );
    }
    sum
}

pub fn get_banana_cost(state: &GameState) -> u64 {
    compute_action_price(
        state.config.banana_target_price,
        state.config.banana_per_turn_decrease,
        state.turns,
        state
            .action_sold
            .get(&ActionType::Banana)
            .unwrap()
            .to_owned(),
        state.config.banana_sell_per_turn,
    )
}

pub fn get_shield_cost(state: &GameState, amount: u64) -> u64 {
    let mut sum = 0;

    for i in 0..amount {
        sum += compute_action_price(
            state.config.shield_target_price,
            state.config.shield_per_turn_decrease,
            state.turns,
            state.action_sold.get(&ActionType::Shield).unwrap() + i,
            state.config.shield_sell_per_turn,
        );
    }
    sum
}

pub fn get_bananas_sorted_by_y(state: &GameState) -> Vec<u64> {
    let mut sorted = state.bananas.clone();

    for i in 0..sorted.len() {
        for j in (i + 1)..sorted.len() {
            if sorted[j] < sorted[i] {
                // Swap using xor operation
                sorted[i] = sorted[j] ^ sorted[i];
                sorted[i] = sorted[i] ^ sorted[j];
                sorted[j] = sorted[j] ^ sorted[i];
            }
        }
    }

    sorted
}

pub fn get_cars_sorted_by_y(state: &GameState) -> Vec<Addr> {
    let mut cars = state.all_cars.clone();

    for i in 0..state.config.num_players {
        for j in (i + 1)..state.config.num_players {
            if state.map_addr_car.get(&cars[j as usize]).unwrap().y
                > state.map_addr_car.get(&cars[i as usize]).unwrap().y
            {
                let temp = cars[i as usize].clone();
                cars[i as usize] = cars[j as usize].clone();
                cars[j as usize] = temp
            }
        }
    }

    cars
}

pub fn get_all_car_data_and_find_car(state: &GameState, addr: Addr) -> (Vec<CarData>, Option<u64>) {
    let mut results = Vec::new();
    let mut found_car_index = None;

    let sorted_cars = get_cars_sorted_by_y(state);
    
    for i in 0..(state.config.num_players) {
        let car_addr = sorted_cars[i as usize].clone();

        if car_addr == addr {
            found_car_index = Some(i as u64);
        }
        let car_data =  state.map_addr_car.get(&car_addr).unwrap().to_owned(); 
        results.push(car_data);
    }

    (results, found_car_index)
}

pub fn compute_action_price(
    target_price: u64,
    per_turn_price_decrease: u64,
    turn_since_start: u64,
    sold: u64,
    sell_per_turn_wad: u64,
) -> u64 {
    1
}

#[cfg(test)]
pub mod tests {
    use cosmwasm_std::Addr;

    use crate::state::GameState;

    use super::{get_cars_sorted_by_y, get_all_car_data_and_find_car};

    #[test]
    fn test_get_cars_sorted_by_y() {
        let state = GameState::for_test();

        println!("before sort: {:?}", state.all_cars.clone());

        let sorted_cars = get_cars_sorted_by_y(&state);

        println!("sorted: {:?}", sorted_cars);
    }

    #[test]
    fn test_get_all_car_and_find_car() {
        let state = GameState::for_test();
        let find_addr = Addr::unchecked("add2");
        let (all_cars, car_index) = get_all_car_data_and_find_car(&state, find_addr);

        println!("all_cars = {:?}", all_cars);
        println!("found_car_index = {:?}", car_index);
    }
}
