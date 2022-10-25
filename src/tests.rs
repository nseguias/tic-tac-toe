#[cfg(test)]
use crate::{contract::instantiate, msg::InstantiateMsg};
use crate::{
    contract::{create_game, join_game, submit_move},
    msg::{CreateGameMsg, GameStatus, JoinGameMsg, SubmitMoveMsg},
    state::{CONFIG, GAME, STATE},
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

    println!("{:?}", state);
    assert_eq!(res_expected, res);
    assert_eq!(res.attributes.len(), 2);
    assert_eq!(state.latest_game_id, 0);
    assert_eq!(config.owner, "instantiatoor");
}

#[test]
fn creating_a_game() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    instantiate(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    let info = mock_info("player_1", &[]);
    let msg = CreateGameMsg {};

    let res = create_game(deps.as_mut(), env, info, msg).unwrap();
    let res_expected: Response = Response::new()
        .add_attribute("action", "create_game")
        .add_attribute("game_id", "0")
        .add_attribute("players", "player_1");

    let state = STATE.load(&deps.storage).unwrap();
    let game = GAME.load(&deps.storage, 0).unwrap();

    assert_eq!(res_expected, res);
    assert_eq!(res.attributes.len(), 3);
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

    assert_eq!(res_expected, res);
    assert_eq!(game.status, GameStatus::InProgress);
    assert_eq!(game.next_turn, Some(Addr::unchecked("player_2")));
    assert_eq!(game.players[1], Addr::unchecked("player_1"));
    assert_eq!(game.players[0], Addr::unchecked("player_2"));
    println!("{:?}", game.moves);
}
