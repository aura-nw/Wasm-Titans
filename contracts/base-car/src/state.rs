use std::{fmt::Display, vec};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<String> = Item::new("owner");
pub const GAME_STATE: Item<GameState> = Item::new("game_state");

// ACTION_SOLD is map of number action has sold
// example: <"shell", 10> meaning action shell has sold with 10 amount
pub const ACTION_SOLD: Map<&str, u64> = Map::new("action_sold");

pub const ALL_CAR_DATA: Map<Addr, CarData> = Map::new("all_car_data");

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
            post_sell_speed: 1,
            init_balance: 17500,
            target_distance: 1000,
            banana_speed_modifier: 0,
            shell_target_price: 0,
            shell_per_turn_decrease: 0,
            shell_sell_per_turn: 0,
            accel_target_price: 0,
            accel_per_turn_decrease: 0,
            accel_sell_per_turn: 0,
            ss_target_price: 0,
            ss_per_turn_decrease: 0, // zero
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
#[derive(Eq, Hash)]
pub enum ActionType {
    Accelerate,
    Shell,
    SuperShell,
    Banana,
    Shield,
}

impl Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ActionType::Accelerate => write!(f, "accelerate"),
            ActionType::Shell => write!(f, "shell"),
            ActionType::SuperShell => write!(f, "super_shell"),
            ActionType::Banana => write!(f, "banana"),
            ActionType::Shield => write!(f, "shield"),
        }
    }
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
    #[warn(dead_code)]
    fn test_car() -> Self {
        Self {
            balance: 1000,
            addr: Addr::unchecked("input"),
            y: 10,
            speed: 2,
            shield: 0,
        }
    }
}

impl CarData {
    pub fn at_start(addr: Addr) -> Self {
        Self {
            balance: 17500,
            addr: addr,
            y: 0,
            speed: 0,
            shield: 0,
        }
    }

    pub fn empty() -> Self {
        Self {
            balance: 0,
            addr: Addr::unchecked(""),
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

    // The current state of the game: pre-start, started, done.
    pub state: State,

    // Game config
    pub config: Config,

    // The banana in play, tracked by their y position.
    pub bananas: Vec<u64>,
}

impl GameState {
    pub fn default() -> Self {
        Self {
            all_cars: Vec::new(),
            turns: 0,
            state: State::Waiting,
            config: Config::default(),
            bananas: Vec::new(),
        }
    }

    pub fn for_test() -> Self {
        let addr1 = Addr::unchecked("addr1");
        let addr2 = Addr::unchecked("addr2");
        let addr3 = Addr::unchecked("addr3");
        let all_cars = vec![addr1.clone(), addr2.clone(), addr3.clone()];

        Self {
            all_cars,
            turns: 3,
            state: State::Active,
            config: Config::default(),
            bananas: Vec::new(),
        }
    }

    pub fn register(&mut self, car_addrs: Vec<Addr>) {
        self.all_cars = car_addrs;
    }

    pub fn total_cars(&self) -> u64 {
        self.all_cars.len() as u64
    }

    pub fn can_play(&self) -> bool {
        return self.all_cars.len() as u64 == self.config.num_players;
    }
}
