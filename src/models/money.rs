use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum MoneyParseError {
    InvalidEuros(String),
    InvalidCents(String),
}

impl std::fmt::Display for MoneyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MoneyParseError::InvalidEuros(err) => write!(f, "Invalid Euros: {err}"),
            MoneyParseError::InvalidCents(err) => write!(f, "Invalid Cents: {err}"),
        }
    }
}
