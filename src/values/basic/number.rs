use cssparser::*;

use crate::{impl_parse_expect, Calc, CustomParseError, Parse};

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

// TODO Support Calc for these types. f32 already supports it (see above)
impl_parse_expect!(i8, expect_integer, as i8);
impl_parse_expect!(i16, expect_integer, as i16);
impl_parse_expect!(i32, expect_integer, as i32);
impl_parse_expect!(i64, expect_integer, as i64);
impl_parse_expect!(i128, expect_integer, as i128);

impl_parse_expect!(u8, expect_integer, as u8);
impl_parse_expect!(u16, expect_integer, as u16);
impl_parse_expect!(u32, expect_integer, as u32);
impl_parse_expect!(u64, expect_integer, as u64);
impl_parse_expect!(u128, expect_integer, as u128);

// impl_parse_expect!(f32, expect_number, as f32);
impl_parse_expect!(f64, expect_number, as f64);
