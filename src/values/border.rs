use crate::{CustomParseError, Length, Parse};
use cssparser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BorderWidth {
    Thin,
    Medium,
    Thick,
    Length(Length),
}

impl Default for BorderWidth {
    fn default() -> Self {
        Self::Medium
    }
}

impl<'i> Parse<'i> for BorderWidth {
    fn parse<'t>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, crate::CustomParseError<'i>>> {
        if let Ok(length) = input.try_parse(|i| Length::parse(i)) {
            return Ok(BorderWidth::Length(length));
        }
        let location = input.current_source_location();
        let ident = input.expect_ident()?;
        match_ignore_ascii_case! { &ident,
            "thin" => Ok(BorderWidth::Thin),
            "medium" => Ok(BorderWidth::Medium),
            "thick" => Ok(BorderWidth::Thick),
            _ => return Err(location.new_unexpected_token_error(Token::Ident(ident.clone())))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub width: BorderWidth,
    pub color: Color,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            width: BorderWidth::Medium,
            color: Color::CurrentColor,
        }
    }
}

impl<'i> Parse<'i> for Border {
    fn parse<'t>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, crate::CustomParseError<'i>>> {
        let mut color = None;
        let mut width = None;
        let mut any = false;

        loop {
            if width.is_none() {
                if let Ok(value) = input.try_parse(|i| BorderWidth::parse(i)) {
                    width = Some(value);
                    any = true;
                }
            }
            if color.is_none() {
                if let Ok(value) = input.try_parse(|i| Color::parse(i)) {
                    color = Some(value);
                    any = true;
                    continue;
                }
            }
            break;
        }

        if any {
            Ok(Border {
                width: width.unwrap_or(BorderWidth::Medium),
                color: color.unwrap_or_else(|| Color::CurrentColor),
            })
        } else {
            Err(input.new_custom_error(CustomParseError::InvalidDeclaration))
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::LengthValue;

    use super::*;

    const VALID_BORDER_WIDTH_LENGTH: &str = "30px";

    #[test]
    fn parse_border_width_length() {
        let mut parser_input = ParserInput::new(&VALID_BORDER_WIDTH_LENGTH);
        let mut parser = Parser::new(&mut parser_input);
        let result = BorderWidth::parse(&mut parser);
        assert_eq!(
            result,
            Ok(BorderWidth::Length(Length::Value(LengthValue::Px(30.0))))
        );
    }

    const VALID_BORDER_WIDTH_THIN: &str = "thin";

    #[test]
    fn parse_border_width_thin() {
        let mut parser_input = ParserInput::new(&VALID_BORDER_WIDTH_THIN);
        let mut parser = Parser::new(&mut parser_input);
        let result = BorderWidth::parse(&mut parser);
        assert_eq!(result, Ok(BorderWidth::Thin));
    }

    const VALID_BORDER_WIDTH_MEDIUM: &str = "medium";

    #[test]
    fn parse_border_width_medium() {
        let mut parser_input = ParserInput::new(&VALID_BORDER_WIDTH_MEDIUM);
        let mut parser = Parser::new(&mut parser_input);
        let result = BorderWidth::parse(&mut parser);
        assert_eq!(result, Ok(BorderWidth::Medium));
    }

    const VALID_BORDER_WIDTH_THICK: &str = "thick";

    #[test]
    fn parse_border_width_thick() {
        let mut parser_input = ParserInput::new(&VALID_BORDER_WIDTH_THICK);
        let mut parser = Parser::new(&mut parser_input);
        let result = BorderWidth::parse(&mut parser);
        assert_eq!(result, Ok(BorderWidth::Thick));
    }

    const VALID_BORDER: &str = "30px red";

    #[test]
    fn parse_border() {
        let mut parser_input = ParserInput::new(&VALID_BORDER);
        let mut parser = Parser::new(&mut parser_input);
        let result = Border::parse(&mut parser);
        assert_eq!(
            result,
            Ok(Border {
                width: BorderWidth::Length(Length::Value(LengthValue::Px(30.0))),
                color: Color::RGBA(RGBA::new(255, 0, 0, 255)),
            })
        );
    }

    const VALID_BORDER_REVERSE: &str = "red 30px";

    #[test]
    fn parse_border_reverse() {
        let mut parser_input = ParserInput::new(&VALID_BORDER_REVERSE);
        let mut parser = Parser::new(&mut parser_input);
        let result = Border::parse(&mut parser);
        assert_eq!(
            result,
            Ok(Border {
                width: BorderWidth::Length(Length::Value(LengthValue::Px(30.0))),
                color: Color::RGBA(RGBA::new(255, 0, 0, 255)),
            })
        );
    }
}
