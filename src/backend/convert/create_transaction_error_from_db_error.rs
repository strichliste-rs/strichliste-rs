use crate::{backend::database::DBError, model::CreateTransactionError};

impl From<DBError> for CreateTransactionError {
    fn from(value: DBError) -> Self {
        Self::StringMessage(value.to_string())
    }
}
