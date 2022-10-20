use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

///
/// Structures
///
#[cw_serde]
pub struct Config {
    /// Admin of this contract
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
    pub moves: Vec<u8>,
    pub starts: Option<Addr>,
}

#[cw_serde]
pub struct Move {
    pub position: u8,
}

///
/// Instantiate
///
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

///
/// Execute
///
#[cw_serde]
pub enum ExecuteMsg {
    CreateGame(CreateGameMsg),
    JoinGame(JoinGameMsg),
    SubmitMove(SubmitMoveMsg),
}

#[cw_serde]
pub struct CreateGameMsg {
    // TO-DO: create a private game?
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
