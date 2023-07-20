#![cfg(test)]


use cosmwasm_std::Empty;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use crate::contract::{execute, instantiate, query};


fn mock_app() -> App {
    App::default()
}

pub fn contract_base_car() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

pub fn contract_car_1() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(car_1::contract::execute, car_1::contract::instantiate, car_1::contract::query);
    Box::new(contract)
}


pub fn contract_car_2() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(car_2::contract::execute, car_2::contract::instantiate, car_2::contract::query);
    Box::new(contract)
}

pub fn contract_car_3() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(car_3::contract::execute, car_3::contract::instantiate, car_3::contract::query);
    Box::new(contract)
}

#[test]
fn test_basic() {    
}