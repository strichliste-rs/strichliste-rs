use leptos::prelude::ServerFnErrorErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Serialize, Deserialize, Clone, Debug)]
pub enum GetUserPreferencesError {
    #[error("Server function error: {0}")]
    ServerFn(ServerFnErrorErr),

    #[error("Database error: {0}")]
    Database(String),
}
