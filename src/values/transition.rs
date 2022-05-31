use crate::{duration::Duration, CustomParseError, Ident, Parse};
use cssparser::{ParseError, ParseErrorKind, Parser};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transition {
    /// List of properties affected by transition
    pub property: String,
    /// Duration of the transition
    pub duration: f32,
    /// Delay of the transition
    pub delay: Option<f32>,
}

impl<'i> Parse<'i> for Transition {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();

        let property = Ident::parse(input)?.into();
        let duration = Duration::parse(input)?.into();
        let delay = if let Ok(delay) = input.try_parse(Duration::parse) {
            Some(delay.into())
        } else {
            None
        };

        if input.is_exhausted() {
            Ok(Self {
                property,
                duration,
                delay,
            })
        } else {
            Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            })
        }
    }
}

impl<'i> Parse<'i> for Vec<Transition> {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_comma_separated(Transition::parse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    macro_rules! transition {
        ($property: expr, $duration: expr, $delay: expr$(,)?) => {{
            Transition {
                property: $property,
                duration: $duration,
                delay: $delay,
            }
        }};
    }

    #[test]
    fn test_success() {
        assert_parse_value!(
            Transition,
            "width 2s",
            transition!(String::from("width"), 2.0, None)
        );

        assert_parse_value!(
            Transition,
            "height 2s 1s",
            transition!(String::from("height"), 2.0, Some(1.0))
        );

        assert_parse_value!(
            Vec<Transition>,
            "height 1s 2s, width 3s 4s, rotation 5s 6s",
            vec![
                transition!(String::from("height"), 1.0, Some(2.0)),
                transition!(String::from("width"), 3.0, Some(4.0)),
                transition!(String::from("rotation"), 5.0, Some(6.0)),
            ]
        );
    }

    #[test]
    fn test_failure() {
        assert_parse_value!(Transition, "height 2s 1s 1s");
        assert_parse_value!(Transition, "1s 2s height");
    }
}