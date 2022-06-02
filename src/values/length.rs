use crate::{
    calc::Calc,
    dimension_percentage::DimensionPercentage,
    error::CustomParseError,
    impl_parse_try_parse,
    traits::{Parse, TryAdd},
    LengthValue,
};
use cssparser::*;

/// https://drafts.csswg.org/css-values-4/#typedef-length-percentage
pub type LengthPercentage = DimensionPercentage<LengthValue>;

impl LengthPercentage {
    pub fn zero() -> LengthPercentage {
        LengthPercentage::px(0.0)
    }

    pub fn px(val: f32) -> LengthPercentage {
        LengthPercentage::Dimension(LengthValue::Px(val))
    }
}

/// `<length-percentage> | auto`
#[derive(Debug, Clone, PartialEq)]
pub enum LengthPercentageOrAuto {
    Auto,
    LengthPercentage(LengthPercentage),
}

impl<'i> Parse<'i> for LengthPercentageOrAuto {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input.try_parse(|i| i.expect_ident_matching("auto")).is_ok() {
            return Ok(LengthPercentageOrAuto::Auto);
        }

        if let Ok(percent) = input.try_parse(|input| LengthPercentage::parse(input)) {
            return Ok(LengthPercentageOrAuto::LengthPercentage(percent));
        }

        Err(input.new_error_for_next_token())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Length {
    Value(LengthValue),
    Calc(Box<Calc<Length>>),
}

impl<'i> Parse<'i> for Length {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match input.try_parse(Calc::parse) {
            Ok(Calc::Value(v)) => return Ok(*v),
            Ok(calc) => return Ok(Length::Calc(Box::new(calc))),
            _ => {}
        }

        let len = LengthValue::parse(input)?;
        Ok(Length::Value(len))
    }
}

impl std::ops::Mul<f32> for Length {
    type Output = Self;

    fn mul(self, other: f32) -> Length {
        match self {
            Length::Value(a) => Length::Value(a * other),
            Length::Calc(a) => Length::Calc(Box::new(*a * other)),
        }
    }
}

impl std::ops::Add<Length> for Length {
    type Output = Self;

    fn add(self, other: Length) -> Length {
        match self.try_add(&other) {
            Some(r) => r,
            None => self.add(other),
        }
    }
}

impl Length {
    pub fn zero() -> Length {
        Length::Value(LengthValue::Px(0.0))
    }

    pub fn px(px: f32) -> Length {
        Length::Value(LengthValue::Px(px))
    }

    pub fn to_px(&self) -> Option<f32> {
        match self {
            Length::Value(a) => a.to_px(),
            _ => None,
        }
    }

    fn add(self, other: Length) -> Length {
        let mut a = self;
        let mut b = other;

        if a == 0.0 {
            return b;
        }

        if b == 0.0 {
            return a;
        }

        if a < 0.0 && b > 0.0 {
            std::mem::swap(&mut a, &mut b);
        }

        match (a, b) {
            (Length::Calc(a), Length::Calc(b)) => return Length::Calc(Box::new(*a + *b)),
            (Length::Calc(calc), b) => {
                if let Calc::Value(a) = *calc {
                    a.add(b)
                } else {
                    Length::Calc(Box::new(Calc::Sum(
                        Box::new((*calc).into()),
                        Box::new(b.into()),
                    )))
                }
            }
            (a, Length::Calc(calc)) => {
                if let Calc::Value(b) = *calc {
                    a.add(*b)
                } else {
                    Length::Calc(Box::new(Calc::Sum(
                        Box::new(a.into()),
                        Box::new((*calc).into()),
                    )))
                }
            }
            (a, b) => Length::Calc(Box::new(Calc::Sum(Box::new(a.into()), Box::new(b.into())))),
        }
    }
}

impl TryAdd<Length> for Length {
    fn try_add(&self, other: &Length) -> Option<Length> {
        match (self, other) {
            (Length::Value(a), Length::Value(b)) => {
                if let Some(res) = a.try_add(b) {
                    Some(Length::Value(res))
                } else {
                    None
                }
            }
            (Length::Calc(a), other) => match &**a {
                Calc::Value(v) => v.try_add(other),
                Calc::Sum(a, b) => {
                    if let Some(res) = Length::Calc(Box::new(*a.clone())).try_add(other) {
                        return Some(res.add(Length::from(*b.clone())));
                    }

                    if let Some(res) = Length::Calc(Box::new(*b.clone())).try_add(other) {
                        return Some(Length::from(*a.clone()).add(res));
                    }

                    None
                }
                _ => None,
            },
            (other, Length::Calc(b)) => match &**b {
                Calc::Value(v) => other.try_add(&*v),
                Calc::Sum(a, b) => {
                    if let Some(res) = other.try_add(&Length::Calc(Box::new(*a.clone()))) {
                        return Some(res.add(Length::from(*b.clone())));
                    }

                    if let Some(res) = other.try_add(&Length::Calc(Box::new(*b.clone()))) {
                        return Some(Length::from(*a.clone()).add(res));
                    }

                    None
                }
                _ => None,
            },
        }
    }
}

impl std::convert::Into<Calc<Length>> for Length {
    fn into(self) -> Calc<Length> {
        match self {
            Length::Calc(c) => *c,
            b => Calc::Value(Box::new(b)),
        }
    }
}

impl std::convert::From<Calc<Length>> for Length {
    fn from(calc: Calc<Length>) -> Length {
        Length::Calc(Box::new(calc))
    }
}

impl std::cmp::PartialEq<f32> for Length {
    fn eq(&self, other: &f32) -> bool {
        match self {
            Length::Value(a) => *a == *other,
            Length::Calc(_) => false,
        }
    }
}

impl std::cmp::PartialOrd<f32> for Length {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        match self {
            Length::Value(a) => a.partial_cmp(other),
            Length::Calc(_) => None,
        }
    }
}

impl std::cmp::PartialOrd<Length> for Length {
    fn partial_cmp(&self, other: &Length) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Length::Value(a), Length::Value(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LengthOrNumber {
    Length(Length),
    Number(f32),
}

impl Default for LengthOrNumber {
    fn default() -> LengthOrNumber {
        LengthOrNumber::Number(0.0)
    }
}

impl_parse_try_parse! {
    LengthOrNumber,
    f32, Length,
}

impl From<f32> for LengthOrNumber {
    fn from(number: f32) -> Self {
        Self::Number(number)
    }
}

impl From<Length> for LengthOrNumber {
    fn from(length: Length) -> Self {
        Self::Length(length)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const VALID_LENGTH_VALUE_PX: &str = "100px";

//     #[test]
//     fn parse_length_value_px() {
//         let mut parser_input = ParserInput::new(&VALID_LENGTH_VALUE_PX);
//         let mut parser = Parser::new(&mut parser_input);
//         let result = LengthValue::parse(&mut parser);
//         assert_eq!(result, Ok(LengthValue::Px(100.0)));
//     }

//     const VALID_LENGTH_VALUE_IN: &str = "100in";

//     #[test]
//     fn parse_length_value_in() {
//         let mut parser_input = ParserInput::new(&VALID_LENGTH_VALUE_IN);
//         let mut parser = Parser::new(&mut parser_input);
//         let result = LengthValue::parse(&mut parser);
//         assert_eq!(result, Ok(LengthValue::In(100.0)));
//     }

//     const VALID_LENGTH_VALUE_CM: &str = "100cm";

//     #[test]
//     fn parse_length_value_cm() {
//         let mut parser_input = ParserInput::new(&VALID_LENGTH_VALUE_CM);
//         let mut parser = Parser::new(&mut parser_input);
//         let result = LengthValue::parse(&mut parser);
//         assert_eq!(result, Ok(LengthValue::Cm(100.0)));
//     }
// }
