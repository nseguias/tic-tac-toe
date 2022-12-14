use crate::{
    msg::{JoinGameMsg, ResignMsg},
    state::{CONFIG, GAME, STATE},
    ContractError,
};
use sha2::{Digest, Sha256};

use crate::msg::{
    Config, CreateGameMsg, ExecuteMsg, Game, GameStatus, InstantiateMsg, State, SubmitMoveMsg,
};
use cosmwasm_std::{entry_point, Addr, DepsMut, Env, MessageInfo, Response};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: deps
            .api
            .addr_validate(&msg.owner.unwrap_or(info.sender.to_string()))?,
    };
    CONFIG.save(deps.storage, &config)?;

    let state = State { latest_game_id: 0 };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", config.owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateGame(data) => create_game(deps, env, info, data),
        ExecuteMsg::JoinGame(data) => join_game(deps, env, info, data),
        ExecuteMsg::SubmitMove(data) => submit_move(deps, env, info, data),
        ExecuteMsg::Resign(data) => resign(deps, env, info, data),
    }
}

pub fn create_game(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: CreateGameMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    // TO-DO: check all edge cases for failure

    // set default values for an Open game and saves it to storage
    let new_game = Game {
        id: state.latest_game_id,
        players: vec![info.sender],
        status: GameStatus::Open,
        moves: vec!["-".to_string(); 9],
        next_turn: None,
        winner: None,
    };
    GAME.save(deps.storage, state.latest_game_id, &new_game)?;

    // increments latest_game_id and saves it to storage
    STATE.save(
        deps.storage,
        &State {
            latest_game_id: state.latest_game_id + 1,
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "create_game")
        .add_attribute("game_id", new_game.id.to_string())
        .add_attribute("players", new_game.players[0].clone()))
}

pub fn join_game(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: JoinGameMsg,
) -> Result<Response, ContractError> {
    let mut game = GAME.load(deps.storage, msg.game_id)?;

    if game.status != GameStatus::Open {
        return Err(ContractError::CantJoinGame {});
    }

    // TO-DO: check all edge cases for failure

    // add second player to players vector
    game.players.push(info.sender.clone());

    // calculate hash of concatenated strings using Sha256
    let hash = Sha256::new()
        .chain_update(game.players[0].to_string())
        .chain_update(info.sender.to_string())
        .finalize();

    if hash[0].leading_zeros() != 0 {
        // if there are no leading 0, it means first bit is 1. Game initiator plays "O" and goes last. Set next_turn to second player's address
        game.next_turn = Some(game.players[1].clone());

        // re-aranging players vector to have second player on position 0
        game.players = vec![game.players[1].clone(), game.players[0].clone()];
    } else {
        // first bit is 1. Game initiator plays "X" and goes first. Set  next_turn to initiator address
        game.next_turn = Some(game.players[0].clone())
    }

    // set game status to InProgress (from Open) and save to storage
    game.status = GameStatus::InProgress;

    GAME.save(deps.storage, msg.game_id, &game)?;

    Ok(Response::new()
        .add_attribute("action", "join_game")
        .add_attribute("game_id", game.id.to_string())
        .add_attribute("X", game.next_turn.unwrap()))
}

pub fn submit_move(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SubmitMoveMsg,
) -> Result<Response, ContractError> {
    let mut game = GAME.load(deps.storage, msg.game_id)?;

    if msg.position < 1 || msg.position > 9 {
        return Err(ContractError::InvalidPosition {
            position: msg.position.to_string(),
        });
    }

    if game.status != GameStatus::InProgress {
        return Err(ContractError::GameNotInProgress {});
    }

    if game.moves[msg.position as usize - 1] != "-" {
        return Err(ContractError::PositionTaken {});
    }

    if game.next_turn != Some(info.sender.clone()) {
        return Err(ContractError::NotYourTurn {});
    }

    // TO-DO: check all edge cases for failure

    // initialize role as String and assign X or O depending on game.players position
    let role: String;
    let opponent: Addr;
    if game.players[0] == info.sender {
        role = "X".to_string();
        opponent = game.players[1].clone();
    } else {
        role = "O".to_string();
        opponent = game.players[0].clone();
    }

    // add player's decision in the correct position with their corresponding letter
    game.moves[msg.position as usize - 1] = role.clone();

    // terminate the game if there're no more possible moves available
    if !game.moves.contains(&"-".to_string()) {
        game.status = GameStatus::Completed;
    }

    GAME.save(deps.storage, msg.game_id, &game)?;

    let winner = check_winner(game.moves.clone());

    // if there's a winner, set the game status to completed
    if winner != None && winner != Some("-".to_string()) {
        game.status = GameStatus::Completed;
    }

    // set winner to player's address
    if winner == Some("O".to_string()) {
        game.winner = Some(game.players[1].clone());
    } else if winner != None && winner != Some("-".to_string()) {
        game.winner = Some(game.players[0].clone());
    }

    // TO-DO: change next_turn address. Would be nice to track opponents address in a variable
    game.next_turn = Some(opponent);
    GAME.save(deps.storage, msg.game_id, &game)?;

    Ok(Response::new()
        .add_attribute("action", "submit_move")
        .add_attribute("game_id", game.id.to_string())
        .add_attribute("position", msg.position.to_string())
        .add_attribute("role", game.moves[msg.position as usize - 1].to_string()))
}

fn check_winner(moves: Vec<String>) -> Option<String> {
    // checks if same role is in the winning positions, returns winning role or None if nobody won.

    // win along horizontal?
    for i in 0..3 {
        if moves[0 + 3 * i] == moves[1 + 3 * i] && moves[0 + 3 * i] == moves[2 + 3 * i] {
            return Some(moves[0 + 3 * i].clone());
        }
    }

    // win along vertical?
    for i in 0..3 {
        if moves[0 + i] == moves[3 + i] && moves[0] == moves[6 + i] {
            return Some(moves[0 + i].clone());
        }
    }

    // win along negative diagonal?
    if moves[0] == moves[4] && moves[0] == moves[8] {
        return Some(moves[4].clone());
    }

    // win along positive diagonal?
    if moves[2] == moves[4] && moves[2] == moves[6] {
        return Some(moves[4].clone());
    }

    // returns None if there's no winner
    None
}

pub fn resign(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ResignMsg,
) -> Result<Response, ContractError> {
    let mut game = GAME.load(deps.storage, msg.game_id)?;

    if game.status != GameStatus::InProgress {
        return Err(ContractError::GameNotInProgress {});
    }

    // set game status to Completed
    game.status = GameStatus::Completed;

    // set winner to the opponent address and save to storage
    if game.players[0] == info.sender {
        game.winner = Some(game.players[1].clone());
    } else {
        game.winner = Some(game.players[0].clone());
    }
    GAME.save(deps.storage, msg.game_id, &game)?;

    // TO-DO: handle unwrap safetly
    Ok(Response::new()
        .add_attribute("action", "resign")
        .add_attribute("game_id", game.id.to_string())
        .add_attribute("winner", game.winner.unwrap()))
}
