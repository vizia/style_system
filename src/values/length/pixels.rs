use crate::{impl_parse, CustomParseError, Length, Parse};
use cssparser::{ParseError, ParseErrorKind};

/// A length value in pixels.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LengthPixels(pub f32);

impl_parse! {
    LengthPixels,

    custom {
        |input| {
            let location = input.current_source_location();

            if let Some(pixels) = input.try_parse(Length::parse)?.to_px() {
                return Ok(Self(pixels));
            }

            Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            })
        }
    }
}
