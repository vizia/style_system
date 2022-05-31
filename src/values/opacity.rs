use crate::{
    macros::{impl_from_newtype, impl_parse_try_parse},
    percentage::Percentage,
    Parse,
};

/// An opacity value in the range of 0 to 1.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Opacity(pub f32);

impl_parse_try_parse!(Opacity => Percentage, f32);
impl_from_newtype!(Opacity => f32, Percentage);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    #[test]
    fn test_success() {
        assert_parse_value!(Opacity, "0%", Opacity(0.0));
        assert_parse_value!(Opacity, "10%", Opacity(0.1));
        assert_parse_value!(Opacity, "20%", Opacity(0.2));
        assert_parse_value!(Opacity, "30%", Opacity(0.3));
        assert_parse_value!(Opacity, "40%", Opacity(0.4));
        assert_parse_value!(Opacity, "50%", Opacity(0.5));
        assert_parse_value!(Opacity, "60%", Opacity(0.6));
        assert_parse_value!(Opacity, "70%", Opacity(0.7));
        assert_parse_value!(Opacity, "80%", Opacity(0.8));
        assert_parse_value!(Opacity, "90%", Opacity(0.9));
        assert_parse_value!(Opacity, "100%", Opacity(1.0));

        assert_parse_value!(Opacity, "0.0", Opacity(0.0));
        assert_parse_value!(Opacity, "0.1", Opacity(0.1));
        assert_parse_value!(Opacity, "0.2", Opacity(0.2));
        assert_parse_value!(Opacity, "0.3", Opacity(0.3));
        assert_parse_value!(Opacity, "0.4", Opacity(0.4));
        assert_parse_value!(Opacity, "0.5", Opacity(0.5));
        assert_parse_value!(Opacity, "0.6", Opacity(0.6));
        assert_parse_value!(Opacity, "0.7", Opacity(0.7));
        assert_parse_value!(Opacity, "0.8", Opacity(0.8));
        assert_parse_value!(Opacity, "0.9", Opacity(0.9));
        assert_parse_value!(Opacity, "1.0", Opacity(1.0));
    }

    #[test]
    fn test_failure() {
        assert_parse_value!(Opacity, "a");
        assert_parse_value!(Opacity, "0px");
        assert_parse_value!(Opacity, "%10");
    }
}
