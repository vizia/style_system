use crate::{impl_from_newtype, traits::Parse, Calc, CustomParseError, Percentage};
use cssparser::{ParseError, ParseErrorKind, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct CalcValue<T>(pub T);

impl<
        'i,
        V: Parse<'i>
            + std::ops::Mul<f32, Output = V>
            + std::ops::Add<V, Output = V>
            + std::cmp::PartialOrd<V>
            + std::convert::Into<Calc<V>>
            + std::convert::From<Calc<V>>
            + std::fmt::Debug,
    > Parse<'i> for CalcValue<V>
{
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();

        if let Ok(Calc::Value(value)) = input.try_parse(Calc::parse) {
            return Ok(Self(*value));
        }

        Err(ParseError {
            kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
            location,
        })
    }
}

impl_from_newtype!(CalcValue<f32> => f32, Percentage);
