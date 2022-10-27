#[cfg(test)]
use crate::{contract::instantiate, msg::InstantiateMsg};
use crate::{
    contract::{create_game, join_game, resign, submit_move},
    msg::{CreateGameMsg, GameStatus, JoinGameMsg, ResignMsg, SubmitMoveMsg},
    state::{CONFIG, GAME, STATE},
    ContractError,
};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Response,
};

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();
    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    let res = instantiate(
        deps.as_mut(),
        mock_env(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    let res_expected: Response = Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", "instantiatoor");

    let state = STATE.load(&deps.storage).unwrap();
    let config = CONFIG.load(&deps.storage).unwrap();

    // attributes, latest_game_id, and contract owner as expected & state
    assert_eq!(res_expected, res);
    assert_eq!(state.latest_game_id, 0);
    assert_eq!(config.owner, "instantiatoor");
}

#[test]
fn creating_a_game() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    // Instantiating
    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    instantiate(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    // Creating a game
    let info = mock_info("player_1", &[]);
    let msg = CreateGameMsg {};

    let res = create_game(deps.as_mut(), env, info, msg).unwrap();
    let res_expected: Response = Response::new()
        .add_attribute("action", "create_game")
        .add_attribute("game_id", "0")
        .add_attribute("players", "player_1");

    let state = STATE.load(&deps.storage).unwrap();
    let game = GAME.load(&deps.storage, 0).unwrap();

    // attributes as expected & game_id correct & latest_game updated
    assert_eq!(res_expected, res);
    assert_eq!(game.id, 0);
    assert_eq!(state.latest_game_id, 1);
}

#[test]
fn joining_a_game() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    // Instantiating
    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    instantiate(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    // Creating a game
    let info = mock_info("player_1", &[]);
    let msg = CreateGameMsg {};

    create_game(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Joining an existing game
    let info = mock_info("player_2", &[]);
    let msg = JoinGameMsg { game_id: 0 };

    let res = join_game(deps.as_mut(), env, info, msg).unwrap();

    let res_expected: Response = Response::new()
        .add_attribute("action", "join_game")
        .add_attribute("game_id", "0")
        .add_attribute("X", "player_2");

    let game = GAME.load(&deps.storage, 0).unwrap();

    // attributes as expected & player_2 next turn & players in the correct order
    assert_eq!(res_expected, res);
    assert_eq!(game.status, GameStatus::InProgress);
    assert_eq!(game.next_turn, Some(Addr::unchecked("player_2")));
    assert_eq!(game.players[1], Addr::unchecked("player_1"));
    assert_eq!(game.players[0], Addr::unchecked("player_2"));
}

#[test]
fn submitting_a_move() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    // Instantiating
    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    instantiate(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    // Creating a game
    let info = mock_info("player_1", &[]);
    let msg = CreateGameMsg {};

    create_game(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Joining an existing game
    let info = mock_info("player_2", &[]);
    let msg = JoinGameMsg { game_id: 0 };

    join_game(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Submitting a move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 5,
    };
    let res = submit_move(deps.as_mut(), env, info, msg).unwrap();

    let res_expected: Response = Response::new()
        .add_attribute("action", "submit_move")
        .add_attribute("game_id", "0")
        .add_attribute("position", "5")
        .add_attribute("role", "X");

    let game = GAME.load(&deps.storage, 0).unwrap();

    // attributes as expected & game still in progress & player_1 next turn & players in the correct order
    assert_eq!(res_expected, res);
    assert_eq!(game.status, GameStatus::InProgress);
    assert_eq!(game.next_turn, Some(Addr::unchecked("player_1")));
    assert_eq!(game.players[1], Addr::unchecked("player_1"));
    assert_eq!(game.players[0], Addr::unchecked("player_2"));
}

#[test]
fn winning_a_game() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    // Instantiating
    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    instantiate(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    // Creating a game
    let info_1 = mock_info("player_1", &[]);
    let msg = CreateGameMsg {};

    create_game(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Joining an existing game
    let info_2 = mock_info("player_2", &[]);
    let msg = JoinGameMsg { game_id: 0 };

    join_game(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 1st move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 1,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 2nd move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 2,
    };
    submit_move(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Submitting 3rd move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 3,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 4th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 4,
    };
    submit_move(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Submitting 5th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 5,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 6th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 6,
    };
    submit_move(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Submitting 7th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 7,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 8th move should fail as the game should be already completed
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 6,
    };
    let res = submit_move(deps.as_mut(), env.clone(), info_1.clone(), msg);

    let game = GAME.load(&deps.storage, 0).unwrap();

    // game status completed & 8th move should throw an error & player_2 wins
    assert_eq!(game.status, GameStatus::Completed);
    assert_eq!(res.unwrap_err(), ContractError::GameNotInProgress {});
    assert_eq!(game.winner.unwrap(), Addr::unchecked("player_2"));
}

#[test]
fn drawing_a_game() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    // Instantiating
    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    instantiate(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    // Creating a game
    let info_1 = mock_info("player_1", &[]);
    let msg = CreateGameMsg {};

    create_game(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Joining an existing game
    let info_2 = mock_info("player_2", &[]);
    let msg = JoinGameMsg { game_id: 0 };

    join_game(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 1st move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 1,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 2nd move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 2,
    };
    submit_move(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Submitting 3rd move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 3,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 4th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 4,
    };
    submit_move(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Submitting 5th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 5,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 6th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 7,
    };
    submit_move(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Submitting 7th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 6,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    // Submitting 8th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 9,
    };
    submit_move(deps.as_mut(), env.clone(), info_1.clone(), msg).unwrap();

    // Submitting 9th move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 8,
    };
    submit_move(deps.as_mut(), env.clone(), info_2.clone(), msg).unwrap();

    let game = GAME.load(&deps.storage, 0).unwrap();

    // game completed & no winner
    assert_eq!(game.status, GameStatus::Completed);
    assert_eq!(game.winner, None);
}

#[test]
fn resigning_after_one_move() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    // Instantiating
    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    instantiate(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    // Creating a game
    let info = mock_info("player_1", &[]);
    let msg = CreateGameMsg {};

    create_game(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Joining an existing game
    let info = mock_info("player_2", &[]);
    let msg = JoinGameMsg { game_id: 0 };

    join_game(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Submitting a move
    let msg = SubmitMoveMsg {
        game_id: 0,
        position: 5,
    };

    submit_move(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ResignMsg { game_id: 0 };

    let res = resign(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();

    let res_expected: Response = Response::new()
        .add_attribute("action", "resign")
        .add_attribute("game_id", "0")
        .add_attribute("winner", "player_1");

    let game = GAME.load(&deps.storage, 0).unwrap();

    // attributes as expected & game completed & player_1 won
    assert_eq!(res_expected, res);
    assert_eq!(game.status, GameStatus::Completed);
    assert_eq!(game.winner.unwrap(), Addr::unchecked("player_1"));
}
