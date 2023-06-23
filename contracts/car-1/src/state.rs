use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const BASE_CAR_ADDR: Item<Addr> = Item::new("base_car_addr");
