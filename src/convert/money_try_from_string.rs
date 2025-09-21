use crate::model::{Money, MoneyParseError};

impl TryFrom<String> for Money {
    type Error = MoneyParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        #[allow(unused_assignments)] // euros is never read
        let string = value.replace(",", ".");

        let (euros, mut cents) = match string.rsplit_once(".") {
            Some(split) => (split.0.to_string(), split.1.to_string()),
            None => (string, 0.to_string()),
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

        let real_euros = match euros.parse::<i64>() {
            Ok(real_euros) => real_euros,
            Err(_) => {
                return Err(MoneyParseError::InvalidEuros(format!(
                    "Failed to parse euros: {}",
                    euros
                )));
            }
        };

        let real_cents = match cents.parse::<i64>() {
            Ok(real_cents) => real_cents,
            Err(_) => {
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
