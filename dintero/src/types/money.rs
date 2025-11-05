//! Module implementation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Currency {
    #[default]
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "USD")]
    Usd,
}

impl Currency {
    pub fn code(&self) -> &str {
        match self {
            Currency::Nok => "NOK",
            Currency::Sek => "SEK",
            Currency::Dkk => "DKK",
            Currency::Eur => "EUR",
            Currency::Usd => "USD",
        }
    }

    pub fn minor_units(&self) -> u32 {
        match self {
            Currency::Nok | Currency::Sek | Currency::Dkk | Currency::Eur | Currency::Usd => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Money {
    pub amount: i64,
}

impl Money {
    pub fn new(amount: i64) -> Self {
        Self { amount }
    }

    pub fn zero() -> Self {
        Self { amount: 0 }
    }

    pub fn from_major(major: i64, currency: Currency) -> Self {
        let minor_units = currency.minor_units();
        let multiplier = 10_i64.pow(minor_units);
        Self { amount: major * multiplier }
    }

    pub fn to_major(&self, currency: Currency) -> f64 {
        let minor_units = currency.minor_units();
        let divisor = 10_f64.powi(minor_units as i32);
        self.amount as f64 / divisor
    }

    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }

    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }

    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }
}

impl From<i64> for Money {
    fn from(amount: i64) -> Self {
        Self { amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_from_major() {
        let money = Money::from_major(100, Currency::Nok);
        assert_eq!(money.amount, 10000);
    }

    #[test]
    fn test_money_to_major() {
        let money = Money::new(12345);
        assert_eq!(money.to_major(Currency::Nok), 123.45);
    }

    #[test]
    fn test_money_predicates() {
        assert!(Money::zero().is_zero());
        assert!(Money::new(100).is_positive());
        assert!(Money::new(-100).is_negative());
    }
}
