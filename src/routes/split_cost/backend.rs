use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::routes::user::CreateTransactionError;

#[cfg(feature = "ssr")]
use {crate::backend::database::DBError, tracing::error};

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

impl FromServerFnError for SplitCostError {
    type Encoder = server_fn::codec::JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self::ServerFn(value)
    }
}

impl From<CreateTransactionError> for SplitCostError {
    fn from(value: CreateTransactionError) -> Self {
        Self::CreateTransactionError(value.to_string())
    }
}

#[cfg(feature = "ssr")]
impl From<DBError> for SplitCostError {
    fn from(value: DBError) -> Self {
        Self::DatabaseError(value.to_string())
    }
}
