use crate::state::{CONFIG, STATE};
#[cfg(test)]
use crate::{contract::instantiate, msg::InstantiateMsg};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Response,
};

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies();

    let instantiate_msg = InstantiateMsg { owner: None };
    let admin_info = mock_info("instantiatoor", &[]);

    let instantiate_res_expected: Response = Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", "instantiatoor");

    let instantiate_res = instantiate(
        deps.as_mut(),
        mock_env(),
        admin_info.clone(),
        instantiate_msg,
    )
    .unwrap();

    let state = STATE.load(&deps.storage).unwrap();
    let config = CONFIG.load(&deps.storage).unwrap();

    assert_eq!(instantiate_res_expected, instantiate_res);
    assert_eq!(instantiate_res.attributes.len(), 2);
    assert_eq!(state.current_game_id, 0);
    assert_eq!(config.owner, "instantiatoor");
}
