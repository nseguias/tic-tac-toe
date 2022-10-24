use crate::{
    msg::JoinGameMsg,
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

    let state = State { current_game_id: 0 };
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

    let new_game = Game {
        id: state.current_game_id,
        players: vec![info.sender],
        status: GameStatus::Open,
        moves: vec!["-".to_string(); 9],
        next_turn: None,
        winner: None,
    };
    GAME.save(deps.storage, 0, &new_game)?;

    STATE.save(
        deps.storage,
        &State {
            current_game_id: state.current_game_id + 1,
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

    // TO-DO: check all edge cases for failure
    game.players.push(info.sender.clone());

    let hash = Sha256::new()
        .chain_update(game.players[0].to_string())
        .chain_update(info.sender.as_str())
        .finalize();
    if hash[0].leading_zeros() != 0 {
        game.next_turn = Some(game.players[1].clone()); // second player starts [O, X]
        game.players = vec![game.players[1].clone(), game.players[0].clone()]; // re-arange players to have X on position 0
    } else {
        game.next_turn = Some(game.players[0].clone()) // first player starts  [X, O]
    }

    game.status = GameStatus::InProgress;

    GAME.save(deps.storage, 0, &game)?;

    Ok(Response::new()
        .add_attribute("action", "join_game")
        .add_attribute("game_id", game.id.to_string())
        .add_attribute("X", game.next_turn.unwrap())) // TO-DO: fix unwrap()
}

pub fn submit_move(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SubmitMoveMsg,
) -> Result<Response, ContractError> {
    let mut game = GAME.load(deps.storage, msg.game_id)?;

    if game.status != GameStatus::InProgress {
        return Err(ContractError::GameNotInProgress {});
    }

    if game.moves[msg.position as usize] != "-" {
        return Err(ContractError::PositionTaken {});
    }

    if game.next_turn != Some(info.sender.clone()) {
        return Err(ContractError::NotYourTurn {});
    }

    if msg.position < 1 || msg.position > 9 {
        return Err(ContractError::InvalidPosition {
            position: msg.position.to_string(),
        });
    }

    // TO-DO: check all edge cases for failure

    // TO-DO: decide how to track moves and roles
    let role: String;
    if game.players[0] == info.sender {
        role = "X".to_string();
    } else {
        role = "O".to_string();
    }

    game.moves[msg.position as usize - 1] = role;

    if !game.moves.contains(&"-".to_string()) {
        game.status = GameStatus::Completed;
    }

    GAME.save(deps.storage, msg.game_id, &game)?;

    let winner = check_winner(game.moves.clone());

    if winner != None {
        game.status = GameStatus::Completed
    }

    if winner == Some(info.sender) {
        // decide how to proclame winner
    }

    Ok(Response::new()
        .add_attribute("action", "submit_move")
        .add_attribute("game_id", game.id.to_string())
        .add_attribute("position", msg.position.to_string())
        .add_attribute("role", game.moves[msg.position as usize - 1].to_string()))
}

fn check_winner(moves: Vec<String>) -> Option<Addr> {
    Some(Addr::unchecked("player_1".to_string()))
}
