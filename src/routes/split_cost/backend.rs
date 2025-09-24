#[cfg(feature = "ssr")]
use crate::{backend::database::DBError, model::SplitCostError};

#[cfg(feature = "ssr")]
impl From<DBError> for SplitCostError {
    fn from(value: DBError) -> Self {
        Self::DatabaseError(value.to_string())
    }
}
