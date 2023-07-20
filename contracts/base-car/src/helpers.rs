use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

use crate::{msg::ExecuteMsg, state::GameState};

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

pub fn get_accel_cost(state: &GameState, amount: u64, sold: u64) -> u64 {
    let mut sum = 0 as u64;
    for i in 0..amount {
        sum += compute_action_price(
            state.config.accel_target_price,
            state.config.accel_per_turn_decrease,
            state.turns,
            sold + i,
            state.config.accel_sell_per_turn,
        );
    }
    sum
}

pub fn get_shell_cost(state: &GameState, amount: u64, sold: u64) -> u64 {
    let mut sum = 0 as u64;

    for i in 0..amount {
        sum += compute_action_price(
            state.config.shell_target_price,
            state.config.shell_per_turn_decrease,
            state.turns,
            sold + i,
            state.config.shell_sell_per_turn,
        );
    }
    sum
}

pub fn get_super_shell_cost(state: &GameState, amount: u64, sold: u64) -> u64 {
    let mut sum = 0 as u64;

    for i in 0..amount {
        sum += compute_action_price(
            state.config.ss_target_price,
            state.config.ss_per_turn_decrease,
            state.turns,
            sold + i,
            state.config.ss_sell_per_turn,
        );
    }
    sum
}

pub fn get_banana_cost(state: &GameState, sold: u64) -> u64 {
    compute_action_price(
        state.config.banana_target_price,
        state.config.banana_per_turn_decrease,
        state.turns,
        sold,
        state.config.banana_sell_per_turn,
    )
}

pub fn get_shield_cost(state: &GameState, amount: u64, sold: u64) -> u64 {
    let mut sum = 0;

    for i in 0..amount {
        sum += compute_action_price(
            state.config.shield_target_price,
            state.config.shield_per_turn_decrease,
            state.turns,
            sold + i,
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
    use crate::{
        helpers::{get_shell_cost, get_super_shell_cost, get_shield_cost, get_banana_cost},
        state::GameState,
    };

    use super::{get_accel_cost, get_bananas_sorted_by_y};

    #[test]
    fn test_get_bananas_sorted_by_y() {
        let test_state = GameState::for_test();

        let res = get_bananas_sorted_by_y(&test_state);
        assert!(res.len() == test_state.bananas.len());
        println!("{:?}", res);
    }

    #[test]
    fn test_get_cost() {
        let test_state = GameState::for_test();

        let accel_cost = get_accel_cost(&test_state, 2, 3);
        assert!(accel_cost != 0);
        println!("{:?}", accel_cost);

        let shell_cost = get_shell_cost(&test_state, 2, 3);
        assert!(shell_cost != 0);
        println!("{:?}", shell_cost);

        let ss_cost = get_super_shell_cost(&test_state, 2, 3);
        assert!(ss_cost != 0);
        println!("{:?}", ss_cost);

        let shield_cost = get_shield_cost(&test_state, 2, 3);
        assert!(shield_cost != 0);
        println!("{:?}", shield_cost);

        let banana_cost = get_banana_cost(&test_state, 2);
        assert!(banana_cost != 0);
        println!("{:?}", banana_cost);
    }
}

