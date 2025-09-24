use crate::model::{CreateTransactionError, SplitCostError};

#[cfg(feature = "ssr")]
use crate::backend::database::DBError;

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
