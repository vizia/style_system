use crate::{impl_from_newtype, traits::Parse, Calc, CustomParseError, Percentage};
use cssparser::{ParseError, ParseErrorKind, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct CalcNumber(pub f32);

impl<'i> Parse<'i> for CalcNumber {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();

        if let Ok(Calc::Number(number)) = input.try_parse(Calc::<f32>::parse) {
            return Ok(Self(number));
        }

        Err(ParseError {
            kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
            location,
        })
    }
}

impl_from_newtype!(CalcNumber => f32, Percentage);
