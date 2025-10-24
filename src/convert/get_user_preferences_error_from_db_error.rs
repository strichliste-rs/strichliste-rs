#![cfg(feature = "ssr")]
use crate::{backend::database::DBError, model::GetUserPreferencesError};

impl From<DBError> for GetUserPreferencesError {
    fn from(value: DBError) -> Self {
        Self::Database(value.to_string())
    }
}
