use crate::{traits::Parse, CustomParseError};
use cssparser::*;

pub enum Display {
    None,
    Flex,
}

impl<'i> Parse<'i> for Display {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        match_ignore_ascii_case! { &*ident,
          "none" => Ok(Display::None),
          "flex" => Ok(Display::Flex),
          _ => Err(location.new_unexpected_token_error(
            cssparser::Token::Ident(ident.clone())
          ))
        }
    }
}
