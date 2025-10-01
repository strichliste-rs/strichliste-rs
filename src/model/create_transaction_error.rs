use leptos::prelude::ServerFnErrorErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::{DatabaseId, UserId};

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum CreateTransactionError {
    #[error("the following users have too little money: {}", .0.join(", "))]
    TooLittleMoneyError(Vec<String>),

    #[error("the following users have too much money: {}", .0.join(", "))]
    TooMuchMoneyError(Vec<String>),

    #[error("Failed to create transaction: {0}")]
    StringMessage(String),

    #[error("server fn error: {0}")]
    ServerFn(ServerFnErrorErr),

    #[error("The article with id {0} does not exist!")]
    ArticleDoesNotExist(DatabaseId),

    #[error("The user with id {0} does not exist!")]
    UserDoesNotExist(UserId),
}

impl CreateTransactionError {
    pub fn new(value: &str) -> Self {
        Self::StringMessage(value.to_string())
    }
}
