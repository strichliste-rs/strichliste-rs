#![cfg(feature = "ssr")]
use crate::{backend::database::DBError, model::SetUserPreferencesError};

impl From<DBError> for SetUserPreferencesError {
    fn from(value: DBError) -> Self {
        Self::Database(value.to_string())
    }
}
