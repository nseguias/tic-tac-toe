use crate::ContractError::Unauthorized;
use cosmwasm_std::StdError;
use serde_json::Error;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not your turn")]
    NotYourTurn {},

    #[error("Position taken")]
    PositionTaken {},

    #[error("Game not in progress")]
    GameNotInProgress {},

    #[error(
        "Position must be an integer between 1 and 9 (inclusive). Your choice was {}",
        position
    )]
    InvalidPosition { position: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

impl From<serde_json::Error> for ContractError {
    fn from(_: Error) -> Self {
        Unauthorized {}
    }
}
