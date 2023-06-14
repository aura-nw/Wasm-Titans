use std::collections::HashMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const OWNER: Item<String> = Item::new("owner");
pub const GAME_STATE: Item<GameState> = Item::new("game_state");

#[cw_serde]
pub struct Config {
    // Number players required in each round
    pub num_players: u64,

    pub post_sell_speed: u64,

    // Initial balance for each car
    pub init_balance: u64,

    // Target distance
    pub target_distance: u64,

    pub banana_speed_modifier: u64,

    // Shell config
    pub shell_target_price: u64,
    pub shell_per_turn_decrease: u64,
    pub shell_sell_per_turn: u64,

    // Accelrate config
    pub accel_target_price: u64,
    pub accel_per_turn_decrease: u64,
    pub accel_sell_per_turn: u64,

    // Super shell config
    pub ss_target_price: u64,
    pub ss_per_turn_decrease: u64,
    pub ss_sell_per_turn: u64,

    // Banana config
    pub banana_target_price: u64,
    pub banana_per_turn_decrease: u64,
    pub banana_sell_per_turn: u64,

    // Shield config
    pub shield_target_price: u64,
    pub shield_per_turn_decrease: u64,
    pub shield_sell_per_turn: u64,
}

impl Config {
    pub fn default() -> Self {
        Self {
            num_players: 3,
            post_sell_speed: 0,
            init_balance: 0,
            target_distance: 0,
            banana_speed_modifier: 0,
            shell_target_price: 0,
            shell_per_turn_decrease: 0,
            shell_sell_per_turn: 0,
            accel_target_price: 0,
            accel_per_turn_decrease: 0,
            accel_sell_per_turn: 0,
            ss_target_price: 0,
            ss_per_turn_decrease: 0,
            ss_sell_per_turn: 0,
            banana_target_price: 0,
            banana_per_turn_decrease: 0,
            banana_sell_per_turn: 0,
            shield_target_price: 0,
            shield_per_turn_decrease: 0,
            shield_sell_per_turn: 0,
        }
    }
}

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

#[cw_serde]
pub struct CarData {
    pub balance: u64,
    pub addr: Addr,
    pub y: u64,
    pub speed: u64,
    pub shield: u64,
}

impl CarData {
    fn at_start(addr: Addr) -> Self {
        Self {
            balance: 0,
            addr: addr,
            y: 0,
            speed: 0,
            shield: 0,
        }
    }
}

#[cw_serde]
pub struct GameState {
    pub all_cars: Vec<Addr>,
    pub turns: u64,

    pub map_addr_car: HashMap<Addr, CarData>,

    pub state: State,
    pub config: Config,
}

impl GameState {
    pub fn default() -> Self {
        Self {
            all_cars: Vec::new(),
            turns: 0,
            map_addr_car: HashMap::new(),
            state: State::Waiting,
            config: Config::default(),
        }
    }

    pub fn register(&mut self, car_addr: Addr) {
        self.all_cars.push(car_addr.clone());
        self.map_addr_car
            .insert(car_addr.clone(), CarData::at_start(car_addr.clone()));
    }

    pub fn total_cars(&self) -> u64 {
        self.all_cars.len() as u64
    }
}
