use crate::{traits::*, CustomParseError};
use cssparser::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomIdent<'i>(pub CowRcStr<'i>);

impl<'i> Parse<'i> for CustomIdent<'i> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        let valid = match_ignore_ascii_case! { &ident,
            "initial" | "inherit" | "unset" | "default" | "revert" => false,
            _ => true
        };

        if !valid {
            return Err(location.new_unexpected_token_error(Token::Ident(ident.clone())));
        }

        Ok(CustomIdent(ident.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_IDENT: &str = "something";
    const INVALID_IDENT: &str = "initial";

    #[test]
    fn parse_valid_custom_ident() {
        let mut parser_input = ParserInput::new(&VALID_IDENT);
        let mut parser = Parser::new(&mut parser_input);
        let result = CustomIdent::parse(&mut parser);
        assert_eq!(result, Ok(CustomIdent(CowRcStr::from("something"))));
    }

    #[test]
    fn parse_invalid_custom_ident() {
        let mut parser_input = ParserInput::new(&INVALID_IDENT);
        let mut parser = Parser::new(&mut parser_input);
        let result = CustomIdent::parse(&mut parser);
        println!("result: {:?}", result);
    }
}
