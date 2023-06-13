use cosmwasm_schema::cw_serde;

pub const PLAYERS_REQUIRED: u16 = 3;
pub const POST_SHELL_SPEED: u64 = 1;
pub const STARTING_BALANCE: u64 = 17500;
pub const FINISH_DISTANCE: u64 = 1000;
pub const BANANA_SPEED_MODIFIER: f64 = 0.5e18;

pub const SHELL_TARGET_PRICE: f64 = 200e18;
pub const SHELL_PER_TURN_DECREASE: f64 = 0.33e18;
pub const SHELL_SELL_PER_TURN: f64 = 0.2e18;

pub const ACCELERATE_TARGET_PRICE: f64 = 10e18;
pub const ACCELERATE_PER_TURN_DECREASE: f64 = 0.33e18;
pub const ACCELERATE_SELL_PER_TURN: f64 = 2e18;

pub const SUPER_SHELL_TARGET_PRICE: f64 = 300e18;
pub const SUPER_SHELL_PER_TURN_DECREASE: f64 = 0.35e18;
pub const SUPER_SHELL_SELL_PER_TURN: f64 = 0.2e18;

pub const BANANA_TARGET_PRICE: f64 = 300e18;
pub const BANANA_PER_TURN_DECREASE: f64 = 0.33e18;
pub const BANANA_SELL_PER_TURN: f64 = 0.2e18;

pub const SHIELD_TAGET_PRICE: f64 = 150e18;
pub const SHIELD_PER_TURN_DECREASE: f64 = 0.33e18;
pub const SHIELD_SELL_PER_TURN: f64 = 0.2e18;

#[cw_serde]
pub enum State {
    Waiting,
    Active,
    Done,
}

#[cw_serde]
pub enum ActionType {
    Accelerate,
    Shell,
    SuperShell,
    Banana,
    Shield,
}
