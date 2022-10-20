use crate::{
    msg::JoinGameMsg,
    state::{CONFIG, GAME, STATE},
    ContractError,
};
use sha2::{Digest, Sha256};

use crate::msg::{
    Config, CreateGameMsg, ExecuteMsg, Game, GameStatus, InstantiateMsg, State, SubmitMoveMsg,
};
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};

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
        moves: vec![0; 9],
        starts: None,
    };

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
        game.starts = Some(game.players[1].clone()); // second player starts [O, X]
    } else {
        game.starts = Some(game.players[0].clone()) // first player starts  [X, O]
    }

    Ok(Response::new()
        .add_attribute("action", "join_game")
        .add_attribute("game_id", game.id.to_string())
        .add_attribute("second_player", game.players[1].clone()))
}

pub fn submit_move(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SubmitMoveMsg,
) -> Result<Response, ContractError> {
    let mut game = GAME.load(deps.storage, msg.game_id)?;

    if game.status == GameStatus::Completed {
        return Err(ContractError::Unauthorized {}); //TO-DO: create custom error
    }

    if game.moves[usize::from(msg.position)] != 0 {
        return Err(ContractError::Unauthorized {}); //TO-DO: create custom error
    }

    //TO-DO: check all edge cases for failure
    let role: u8;
    if Some(info.sender.clone()) == game.starts {
        role = 0;
    } else {
        role = 1;
    }
    game.players.push(info.sender);
    game.moves[usize::from(msg.position)] = role;

    GAME.save(deps.storage, msg.game_id, &game)?;

    Ok(Response::new()
        .add_attribute("action", "submit_move")
        .add_attribute("game_id", game.id.to_string())
        .add_attribute("move", game.moves.last().unwrap_or(&0).to_string())
        .add_attribute("amount", info.funds[0].amount))
}
