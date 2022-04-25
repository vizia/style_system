use cssparser::*;

use crate::{CustomParseError, Length, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadow {
    pub color: Color,
    x_offset: Length,
    y_offset: Length,
    blur: Length,
    spread: Length,
    inset: bool,
}

impl<'i> Parse<'i> for BoxShadow {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut color = None;
        let mut lengths = None;
        let mut inset = false;

        loop {
            if !inset {
                if input
                    .try_parse(|input| input.expect_ident_matching("inset"))
                    .is_ok()
                {
                    inset = true;
                    continue;
                }
            }

            if lengths.is_none() {
                let value = input.try_parse::<_, _, ParseError<CustomParseError>>(|input| {
                    let horizontal = Length::parse(input)?;
                    let vertical = Length::parse(input)?;
                    let blur = input.try_parse(Length::parse).unwrap_or(Length::zero());
                    let spread = input.try_parse(Length::parse).unwrap_or(Length::zero());
                    Ok((horizontal, vertical, blur, spread))
                });

                if let Ok(value) = value {
                    lengths = Some(value);
                    continue;
                }
            }

            if color.is_none() {
                if let Ok(value) = input.try_parse(Color::parse) {
                    color = Some(value);
                    continue;
                }
            }

            break;
        }

        let lengths = lengths.ok_or(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))?;

        Ok(BoxShadow {
            color: color.unwrap_or(Color::CurrentColor),
            x_offset: lengths.0,
            y_offset: lengths.1,
            blur: lengths.2,
            spread: lengths.3,
            inset,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::LengthValue;

    use super::*;

    const VALID_BOX_SHADOW: &str = "inset 5px -5px 7px 14px #D9DB85";

    #[test]
    fn parse_box_shadow() {
        let mut parser_input = ParserInput::new(&VALID_BOX_SHADOW);
        let mut parser = Parser::new(&mut parser_input);
        let result = BoxShadow::parse(&mut parser);
        assert_eq!(
            result,
            Ok(BoxShadow {
                color: Color::RGBA(RGBA::new(217, 219, 133, 255)),
                x_offset: Length::Value(LengthValue::Px(5.0)),
                y_offset: Length::Value(LengthValue::Px(-5.0)),
                blur: Length::Value(LengthValue::Px(7.0)),
                spread: Length::Value(LengthValue::Px(14.0)),
                inset: true
            })
        );
    }
}
