use crate::model::UserId;

impl From<i64> for UserId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
