use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Register {},

    Play {},

    BuyShell {},

    BuyAcceleration {},

    BuyBanana {},

    BuyShield {},

    BuySuperShell {},
}

#[cw_serde]
pub struct GetAllCarDataReponse {}

#[cw_serde]
pub struct GetAllBananasResponse {}

#[cw_serde]
pub struct GetAccelerateCostResponse {}

#[cw_serde]
pub struct GetShellCostResponse {}

#[cw_serde]
pub struct GetSuperShellCostResponse {}

#[cw_serde]
pub struct GetBananaCostResponse {}

#[cw_serde]
pub struct GetShieldCostResponse {} 

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetAllCarDataReponse)]
    GetAllCarData {},

    #[returns(GetAllBananasResponse)]
    GetAllBananas{},

    #[returns(GetAccelerateCostResponse)]
    GetAccelerateCost{},

    #[returns(GetShellCostResponse)]
    GetShellCost{},

    #[returns(GetSuperShellCostResponse)]
    GetSuperShellCost{},

    #[returns(GetBananaCostResponse)]
    GetBananaCost{},

    #[returns(GetShieldCostResponse)]
    GetShieldCost,
}
