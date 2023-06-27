use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::CarData;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { car_addrs: Vec<Addr> },

    Reset {},

    Play { turns_to_play: u64 },

    BuyShell { amount: u64 },

    BuyAccelerate { amount: u64 },

    BuyBanana {},

    BuyShield { amount: u64 },

    BuySuperShell { amount: u64 },
}

#[cw_serde]
pub struct AllCarDataReponse {
    pub all_cars: Vec<(Addr, CarData)>,
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OwnerResponse)]
    GetOwner {},

    #[returns(AllCarDataReponse)]
    GetAllCarData {},
}
