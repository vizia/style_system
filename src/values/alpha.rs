use crate::{impl_from_newtype, impl_parse_try_parse, traits::Parse, Percentage};

#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue(pub f32);

impl_parse_try_parse!(AlphaValue => Percentage, f32);
impl_from_newtype!(AlphaValue => f32, Percentage);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    #[test]
    fn parse_alpha_number() {
        assert_parse_value!(AlphaValue, "0.3", AlphaValue(0.3));
    }

    #[test]
    fn parse_alpha_percentage() {
        assert_parse_value!(AlphaValue, "30%", AlphaValue(0.3));
    }
}
