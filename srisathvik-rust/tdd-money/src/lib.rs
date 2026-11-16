use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    Usd,
    Chf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    amount: i32,
    currency: Currency,
}

impl Money {
    pub fn dollar(amount: i32) -> Self {
        Self {
            amount,
            currency: Currency::Usd,
        }
    }

    pub fn franc(amount: i32) -> Self {
        Self {
            amount,
            currency: Currency::Chf,
        }
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn times(self, multiplier: i32) -> Self {
        Self {
            amount: self.amount * multiplier,
            ..self
        }
    }

    pub fn plus(self, other: Self) -> Option<Self> {
        if self.currency != other.currency {
            return None;
        }

        Some(Self {
            amount: self.amount + other.amount,
            currency: self.currency,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Rate {
    numerator: i32,
    denominator: i32,
}

#[derive(Debug, Default)]
pub struct Bank {
    rates: HashMap<(Currency, Currency), Rate>,
}

impl Bank {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rate(&mut self, from: Currency, to: Currency, numerator: i32, denominator: i32) {
        assert!(denominator != 0, "rate denominator cannot be zero");

        self.rates.insert(
            (from, to),
            Rate {
                numerator,
                denominator,
            },
        );
    }

    pub fn convert(&self, money: Money, target: Currency) -> Option<Money> {
        if money.currency == target {
            return Some(money);
        }

        let rate = self.rates.get(&(money.currency, target))?;

        Some(Money {
            amount: money.amount * rate.numerator / rate.denominator,
            currency: target,
        })
    }

    pub fn add(&self, left: Money, right: Money, target: Currency) -> Option<Money> {
        let left = self.convert(left, target)?;
        let right = self.convert(right, target)?;

        left.plus(right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_dollars_and_francs() {
        assert_eq!(Money::dollar(5).amount(), 5);
        assert_eq!(Money::dollar(5).currency(), Currency::Usd);
        assert_eq!(Money::franc(5).currency(), Currency::Chf);
    }

    #[test]
    fn multiplies_money_without_changing_original() {
        let five = Money::dollar(5);

        let product = five.times(2);

        assert_eq!(product, Money::dollar(10));
        assert_eq!(five, Money::dollar(5));
    }

    #[test]
    fn compares_amount_and_currency() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_ne!(Money::dollar(5), Money::franc(5));
    }

    #[test]
    fn adds_money_with_same_currency() {
        let result = Money::dollar(5).plus(Money::dollar(10));

        assert_eq!(result, Some(Money::dollar(15)));
    }

    #[test]
    fn direct_addition_rejects_different_currencies() {
        let result = Money::dollar(5).plus(Money::franc(10));

        assert_eq!(result, None);
    }

    #[test]
    fn converts_francs_to_dollars() {
        let mut bank = Bank::new();
        bank.add_rate(Currency::Chf, Currency::Usd, 1, 2);

        let result = bank.convert(Money::franc(10), Currency::Usd);

        assert_eq!(result, Some(Money::dollar(5)));
    }

    #[test]
    fn adds_dollars_and_francs_using_bank_rate() {
        let mut bank = Bank::new();
        bank.add_rate(Currency::Chf, Currency::Usd, 1, 2);

        let result = bank.add(Money::dollar(5), Money::franc(10), Currency::Usd);

        assert_eq!(result, Some(Money::dollar(10)));
    }

    #[test]
    fn conversion_fails_when_rate_is_missing() {
        let bank = Bank::new();

        let result = bank.convert(Money::franc(10), Currency::Usd);

        assert_eq!(result, None);
    }
}
