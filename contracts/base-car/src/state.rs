use std::{collections::HashMap, vec};

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
#[derive(Eq, Hash)]
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
    pub fn at_start(addr: Addr) -> Self {
        Self {
            balance: 0,
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

    pub map_addr_car: HashMap<Addr, CarData>,

    // The current state of the game: pre-start, started, done.
    pub state: State,

    // Game config
    pub config: Config,
    pub action_sold: HashMap<ActionType, u64>,

    // The banana in play, tracked by their y position.
    pub bananas: Vec<u64>,
}

impl GameState {
    pub fn default() -> Self {
        Self {
            all_cars: Vec::new(),
            turns: 0,
            map_addr_car: HashMap::new(),
            state: State::Waiting,
            config: Config::default(),
            action_sold: HashMap::new(),
            bananas: Vec::new(),
        }
    }

    pub fn for_test() -> Self {
        let addr1 = Addr::unchecked("addr1");
        let addr2 = Addr::unchecked("addr2");
        let addr3 = Addr::unchecked("addr3");

        let car_data1 = CarData{
            balance: 1000,
            addr: addr1.clone(),
            y: 278,
            speed: 5,
            shield: 1,
        };

        let car_data2 = CarData {
            balance: 1331,
            addr: addr2.clone(),
            y: 312,
            speed: 3,
            shield: 1,
        };

        let car_data3 = CarData {
            balance: 1112,
            addr: addr3.clone(),
            y: 365,
            speed: 3,
            shield: 0,
        };

        let all_cars = vec![addr1.clone(), addr2.clone(), addr3.clone()];
        let mut map_addr_car = HashMap::new();

        map_addr_car.insert(addr1, car_data1);
        map_addr_car.insert(addr2, car_data2);
        map_addr_car.insert(addr3, car_data3);

        Self {
            all_cars,
            turns: 3,
            map_addr_car,
            state: State::Active,
            config: Config::default(),
            action_sold: HashMap::new(),
            bananas: Vec::new(),
        }
    }

    pub fn all_car_data(&self) -> Vec<CarData> {
        self.map_addr_car.values().cloned().collect()
    }

    pub fn register(&mut self, car_addr: Addr) {
        self.all_cars.push(car_addr.clone());
        self.map_addr_car
            .insert(car_addr.clone(), CarData::at_start(car_addr.clone()));
    }

    pub fn total_cars(&self) -> u64 {
        self.all_cars.len() as u64
    }

    pub fn can_play(&self) -> bool {
        return self.all_cars.len() as u64 == self.config.num_players
    }
}
