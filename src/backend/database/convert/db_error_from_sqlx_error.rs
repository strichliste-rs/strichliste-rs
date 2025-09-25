use crate::backend::database::DBError;

impl From<sqlx::Error> for DBError {
    fn from(value: sqlx::Error) -> Self {
        DBError::new(value)
    }
}
