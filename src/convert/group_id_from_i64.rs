use crate::model::GroupId;

impl From<i64> for GroupId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
