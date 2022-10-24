use std::ops::Add;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}

#[cw_serde]
pub struct State {
    pub current_game_id: u64,
}

#[cw_serde]
pub enum GameStatus {
    Open,
    InProgress,
    Completed,
}

#[cw_serde]
pub struct Game {
    pub id: u64,
    pub players: Vec<Addr>,
    pub status: GameStatus,
    pub moves: Vec<String>,
    pub next_turn: Option<Addr>,
    pub winner: Option<Addr>,
}

#[cw_serde]
pub struct Move {
    pub position: u8,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateGame(CreateGameMsg),
    JoinGame(JoinGameMsg),
    SubmitMove(SubmitMoveMsg),
}

#[cw_serde]
pub struct CreateGameMsg {
    // Feature: create a private game passing opponent Addr
}

#[cw_serde]
pub struct JoinGameMsg {
    pub game_id: u64,
}

#[cw_serde]
pub struct SubmitMoveMsg {
    pub game_id: u64,
    pub position: u8,
}
