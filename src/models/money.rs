use std::ops::{Neg, Sub};

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

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct Money {
    pub value: i64,
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Money {
            value: self.value - rhs.value,
        }
    }
}

impl Neg for Money {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Money { value: -self.value }
    }
}

impl Money {
    pub fn format_value(value: i64) -> String {
        format!("{:.2}", value as f64 / 100.0)
    }

    pub fn format(&self) -> String {
        Money::format_value(self.value)
    }

    pub fn format_eur_value(value: i64) -> String {
        format!("{:.2}€", value as f64 / 100.0)
    }

    pub fn format_eur_diff_value(value: i64) -> String {
        match value > 0 {
            true => format!("+{}", Money::format_eur_value(value)),
            false => Money::format_eur_value(value).to_string(),
        }
        .to_string()
    }

    pub fn format_eur(&self) -> String {
        Money::format_eur_value(self.value)
    }

    pub fn format_eur_diff(&self) -> String {
        Money::format_eur_diff_value(self.value)
    }
}

impl From<u64> for Money {
    fn from(value: u64) -> Self {
        Self {
            value: value as i64,
        }
    }
}
impl From<i64> for Money {
    fn from(value: i64) -> Self {
        Self { value }
    }
}

impl TryFrom<String> for Money {
    type Error = MoneyParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        #[allow(unused_assignments)] // euros is never read
        let (mut euros, mut cents): (String, String) = (0.to_string(), 0.to_string());

        let string = value.replace(",", ".");

        let (euros, cents) = match string.rsplit_once("."){
            Some((euros, cents)) => (euros, cents),
            None => string
        };

        if euros.is_empty() {
            return Err(MoneyParseError::InvalidEuros(
                "Euros are empty!".to_string(),
            ));
        }

        if cents.is_empty() {
            return Err(MoneyParseError::InvalidCents(
                "Cents are empty!".to_string(),
            ));
        }

        if cents.len() > 2 {
            cents.truncate(2);
        }

        if cents.len() < 2 {
            cents.push('0');
        }

        let real_euros = match euros.parse::<i64>(){
            Ok(real_euros) => real_euros,
            Err(err)=>{
                return Err(MoneyParseError::InvalidEuros(format!(
                "Failed to parse euros: {}",
                euros
            )));
            }
        };

        let real_cents = match cents.parse::<i64>(){
            Ok(real_cents) => real_cents,
            Err(err) =>{
                return Err(MoneyParseError::InvalidCents(format!(
                "Failed to parse cents: {}",
                cents
            )));
            }
        };

        let final_cents = real_euros * 100 + real_cents;

        Ok(Money { value: final_cents })
    }
}
