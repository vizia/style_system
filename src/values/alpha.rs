use super::percentage::NumberOrPercentage;
use crate::{traits::Parse, CustomParseError};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue(pub f32);

impl<'i> Parse<'i> for AlphaValue {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match NumberOrPercentage::parse(input)? {
            NumberOrPercentage::Percentage(percent) => Ok(AlphaValue(percent.0)),
            NumberOrPercentage::Number(number) => Ok(AlphaValue(number)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const VALID_ALPHA_NUM: &str = "0.3";

    #[test]
    fn parse_alpha_number() {
        let mut parser_input = ParserInput::new(&VALID_ALPHA_NUM);
        let mut parser = Parser::new(&mut parser_input);
        let result = AlphaValue::parse(&mut parser);
        assert_eq!(result, Ok(AlphaValue(0.3)));
    }

    const VALID_ALPHA_PERCENT: &str = "30%";

    #[test]
    fn parse_alpha_percentage() {
        let mut parser_input = ParserInput::new(&VALID_ALPHA_PERCENT);
        let mut parser = Parser::new(&mut parser_input);
        let result = AlphaValue::parse(&mut parser);
        assert_eq!(result, Ok(AlphaValue(0.3)));
    }
}
