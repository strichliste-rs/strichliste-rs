use crate::model::Money;

impl From<i64> for Money {
    fn from(value: i64) -> Self {
        Self { value }
    }
}
