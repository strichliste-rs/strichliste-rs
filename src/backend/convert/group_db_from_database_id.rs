use crate::{backend::database::GroupDB, models::DatabaseId};

impl From<DatabaseId> for GroupDB {
    fn from(id: DatabaseId) -> Self {
        GroupDB { id }
    }
}
