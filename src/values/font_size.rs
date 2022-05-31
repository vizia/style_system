use crate::{
    macros::{impl_from_newtype, impl_parse_try_parse},
    AbsoluteFontSize, Parse,
};

/// A font size value.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FontSize(pub f32);

impl_parse_try_parse!(FontSize => AbsoluteFontSize, f32);
impl_from_newtype!(FontSize => f32);

impl From<AbsoluteFontSize> for FontSize {
    fn from(absolute_font_size: AbsoluteFontSize) -> Self {
        match absolute_font_size {
            AbsoluteFontSize::XXSmall => FontSize(8.0),
            AbsoluteFontSize::XSmall => FontSize(10.0),
            AbsoluteFontSize::Small => FontSize(12.0),
            AbsoluteFontSize::Medium => FontSize(14.0),
            AbsoluteFontSize::Large => FontSize(16.0),
            AbsoluteFontSize::XLarge => FontSize(18.0),
            AbsoluteFontSize::XXLarge => FontSize(20.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    #[test]
    fn test_success() {
        assert_parse_value!(FontSize, "8", FontSize(8.0));
        assert_parse_value!(FontSize, "10", FontSize(10.0));
        assert_parse_value!(FontSize, "12", FontSize(12.0));
        assert_parse_value!(FontSize, "14", FontSize(14.0));
        assert_parse_value!(FontSize, "16", FontSize(16.0));

        assert_parse_value!(FontSize, "small", FontSize(12.0));
        assert_parse_value!(FontSize, "medium", FontSize(14.0));
        assert_parse_value!(FontSize, "large", FontSize(16.0));

        assert_parse_value!(FontSize, "16.123", FontSize(16.123));
        assert_parse_value!(FontSize, "0.123", FontSize(0.123));
    }
}
