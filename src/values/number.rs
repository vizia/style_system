use cssparser::*;

use crate::{Calc, CustomParseError, Parse};

impl<'i> Parse<'i> for f32 {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match input.try_parse(Calc::parse) {
            Ok(Calc::Value(v)) => return Ok(*v),
            Ok(Calc::Number(n)) => return Ok(n),
            // Numbers are always compatible, so they will always compute to a value.
            Ok(_) => unreachable!(),
            _ => {}
        }

        if let Ok(number) = input.try_parse(|input| input.expect_number()) {
            return Ok(number);
        }

        Err(input.new_error_for_next_token())
    }
}

impl std::convert::Into<Calc<f32>> for f32 {
    fn into(self) -> Calc<f32> {
        Calc::Value(Box::new(self))
    }
}

impl std::convert::From<Calc<f32>> for f32 {
    fn from(calc: Calc<f32>) -> f32 {
        match calc {
            Calc::Value(v) => *v,
            _ => unreachable!(),
        }
    }
}
