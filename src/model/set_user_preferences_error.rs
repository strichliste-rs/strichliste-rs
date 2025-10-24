use leptos::prelude::ServerFnErrorErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::UserId;

#[derive(Error, Serialize, Deserialize, Clone, Debug)]
pub enum SetUserPreferencesError {
    #[error("Server function error: {0}")]
    ServerFn(ServerFnErrorErr),

    #[error("Database error: {0}")]
    Database(String),

    #[error("User with id '{}' does not exist", .0)]
    UserDoesNotExist(UserId),
}
