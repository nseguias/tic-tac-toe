use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Config {
    // smart contract owner
    pub owner: Addr,
}

#[cw_serde]
pub struct State {
    pub latest_game_id: u64,
}

#[cw_serde]
pub enum GameStatus {
    Open,
    InProgress,
    Completed,
}

#[cw_serde]
pub struct Game {
    pub id: u64,                 // game id to be able to handle multiple games at once
    pub players: Vec<Addr>, // vector of players' addresses, index 0 contains the address of player "X" who goes first
    pub status: GameStatus, // track game status
    pub moves: Vec<String>, // vector that contains both player's moves in 1-9 board (index 0-8)
    pub next_turn: Option<Addr>, // tracks who plays next
    pub winner: Option<Addr>, // None as long as the game is Open or in Progress, contains the address of the winner once game is Completed
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
    Resign(ResignMsg),
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
    // position goes from 1 to 9, being 1 top left and 9 bottom right
    pub position: u8,
}

#[cw_serde]
pub struct ResignMsg {
    pub game_id: u64,
}
