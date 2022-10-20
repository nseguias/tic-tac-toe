use cw_storage_plus::{Item, Map};

use crate::msg::{Config, Game, State};

pub const CONFIG: Item<Config> = Item::new("config");

pub const STATE: Item<State> = Item::new("state");

pub const GAME: Map<u64, Game> = Map::new("game_state");
