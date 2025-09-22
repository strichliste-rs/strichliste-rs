use crate::{backend::database::GroupDB, model::GroupId};

impl From<&GroupDB> for GroupId {
    fn from(value: &GroupDB) -> Self {
        Self(value.id)
    }
}
