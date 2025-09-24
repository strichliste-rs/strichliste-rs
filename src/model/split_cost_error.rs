use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum SplitCostError {
    #[error("Server function error: {0}")]
    ServerFn(ServerFnErrorErr),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Failed to parse money: {0}")]
    MoneyParseError(String),

    #[error("Money error: {0}")]
    MoneyError(String),

    #[error("User with nickname '{0}' does not exist!")]
    UserDoesNotExistError(String),

    #[error("Failed to create transaction: {0}")]
    CreateTransactionError(String),

    #[error("{0} may not be empty")]
    MayNotBeEmptyError(String),
}
