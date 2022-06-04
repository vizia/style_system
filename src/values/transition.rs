use crate::{duration::Duration, CustomParseError, Ident, Parse};
use cssparser::{ParseError, ParseErrorKind, Parser};

/// Defines a transition that allows to change property values smoothly, over a given duration.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transition {
    /// A list of properties affected by transition.
    pub property: String,
    /// The duration of the transition.
    pub duration: f32,
    /// The delay of the transition.
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
    use crate::tests::{assert_parse, transition};

    assert_parse! {
        Transition, assert_transition,

        custom {
            success {
                "width 2s" => transition!(String::from("width"), 2.0, None),
                "height 2s 1s" => transition!(String::from("height"), 2.0, Some(1.0)),
            }

            failure {
                "height 2s 1s 1s",
                "1s 2s height",
            }
        }
    }

    assert_parse!(
        Vec<Transition>, assert_transitions,

        custom {
            success {
                "height 1s 2s, width 3s 4s, rotation 5s 6s" => vec![
                    transition!(String::from("height"), 1.0, Some(2.0)),
                    transition!(String::from("width"), 3.0, Some(4.0)),
                    transition!(String::from("rotation"), 5.0, Some(6.0)),
                ],
            }

            failure {
                "height, width, rotation",
            }
        }
    );
}
