use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub base_car_addr: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    TakeTurn {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
