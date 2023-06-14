use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Register { car_addr: Addr },

    Play {},

    BuyShell {},

    BuyAccelerate {},

    BuyBanana {},

    BuyShield {},

    BuySuperShell {},
}

#[cw_serde]
pub struct AllCarDataReponse {}

#[cw_serde]
pub struct AllBananasResponse {}

#[cw_serde]
pub struct AccelerateCostResponse {
    pub cost: u64,
}

#[cw_serde]
pub struct ShellCostResponse {
    pub cost: u64,
}

#[cw_serde]
pub struct SuperShellCostResponse {
    pub cost: u64,
}

#[cw_serde]
pub struct BananaCostResponse {
    pub cost: u64,
}

#[cw_serde]
pub struct ShieldCostResponse {
    pub cost: u64,
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OwnerResponse)]
    GetOwner {},

    #[returns(AllCarDataReponse)]
    GetAllCarData {},

    #[returns(AllBananasResponse)]
    GetAllBananas {},

    #[returns(AccelerateCostResponse)]
    GetAccelerateCost {},

    #[returns(ShellCostResponse)]
    GetShellCost {},

    #[returns(SuperShellCostResponse)]
    GetSuperShellCost {},

    #[returns(BananaCostResponse)]
    GetBananaCost {},

    #[returns(ShieldCostResponse)]
    GetShieldCost,
}
