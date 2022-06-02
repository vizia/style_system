use crate::{impl_parse_expect, Calc, Parse};
use cssparser::Token;

/// A percentage value.
#[derive(Debug, Clone, PartialEq)]
pub struct Percentage(pub f32);

impl_parse_expect! {
    Percentage,
    Token::Percentage { unit_value, .. } => Percentage(*unit_value),
}

impl std::convert::Into<Calc<Percentage>> for Percentage {
    fn into(self) -> Calc<Percentage> {
        Calc::Value(Box::new(self))
    }
}

impl std::convert::From<Calc<Percentage>> for Percentage {
    fn from(calc: Calc<Percentage>) -> Percentage {
        match calc {
            Calc::Value(v) => *v,
            _ => unreachable!(),
        }
    }
}

impl std::ops::Mul<f32> for Percentage {
    type Output = Self;

    fn mul(self, other: f32) -> Percentage {
        Percentage(self.0 * other)
    }
}

impl std::ops::Add<Percentage> for Percentage {
    type Output = Self;

    fn add(self, other: Percentage) -> Percentage {
        Percentage(self.0 + other.0)
    }
}

impl std::cmp::PartialEq<f32> for Percentage {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
}

impl std::cmp::PartialOrd<f32> for Percentage {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl std::cmp::PartialOrd<Percentage> for Percentage {
    fn partial_cmp(&self, other: &Percentage) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_percentages;

    assert_parse_percentages! {
        Percentage, parse_percentages,
        Percentage,
    }
}
