use crate::{CustomParseError, Parse};
use cssparser::{ParseError, Parser};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect<T>(
    /// The first value.
    pub T,
    /// The second value.
    pub T,
    /// The third value.
    pub T,
    /// The fourth value.
    pub T,
);

impl<'i, T> Parse<'i> for Rect<T>
where
    T: Parse<'i> + Clone,
{
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        let first = T::parse(input)?;

        let second = if let Ok(second) = input.try_parse(T::parse) {
            second
        } else {
            return Ok(Self(first.clone(), first.clone(), first.clone(), first));
        };

        let third = if let Ok(third) = input.try_parse(T::parse) {
            third
        } else {
            return Ok(Self(first.clone(), second.clone(), first, second));
        };

        let fourth = if let Ok(fourth) = input.try_parse(T::parse) {
            fourth
        } else {
            return Ok(Self(first, second.clone(), third, second));
        };

        if input.is_exhausted() {
            Ok(Self(first, second, third, fourth))
        } else {
            Err(cssparser::ParseError {
                kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                location,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    #[test]
    fn test_sucess() {
        assert_parse_value!(Rect<u8>, "1", Rect(1, 1, 1, 1));
        assert_parse_value!(Rect<u8>, "1 2", Rect(1, 2, 1, 2));
        assert_parse_value!(Rect<u8>, "1 2 3", Rect(1, 2, 3, 2));
        assert_parse_value!(Rect<u8>, "1 2 3 4", Rect(1, 2, 3, 4));
    }

    #[test]
    fn test_failure() {
        assert_parse_value!(Rect<u8>, "1 2 3 4 5");
        assert_parse_value!(Rect<u8>, "abc");
    }
}
