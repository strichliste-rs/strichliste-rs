use crate::{backend::database::GroupDB, model::GroupId};

impl From<GroupDB> for GroupId {
    fn from(val: GroupDB) -> Self {
        GroupId(val.id)
    }
}
