use std::ops::{AddAssign, Neg, Sub};

use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default,
)]
pub struct Money {
    pub value: i64,
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

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}
