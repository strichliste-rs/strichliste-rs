use crate::model::{CreateTransactionError, SplitCostError};
impl From<CreateTransactionError> for SplitCostError {
    fn from(value: CreateTransactionError) -> Self {
        Self::CreateTransactionError(value.to_string())
    }
}
