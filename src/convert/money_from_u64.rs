use crate::model::Money;

impl From<u64> for Money {
    fn from(value: u64) -> Self {
        Self {
            value: value as i64,
        }
    }
}
