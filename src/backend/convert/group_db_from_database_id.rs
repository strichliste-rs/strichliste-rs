use crate::{backend::database::GroupDB, model::DatabaseId};

impl From<DatabaseId> for GroupDB {
    fn from(id: DatabaseId) -> Self {
        GroupDB { id }
    }
}
