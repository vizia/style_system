use crate::{
    auto::Auto, length::Length, macros::impl_parse_try_parse, percentage::Percentage,
    stretch::Stretch, Parse,
};
pub use morphorm::Units;

impl_parse_try_parse!(Units => Auto, Stretch, Percentage, Length);

impl From<Auto> for Units {
    fn from(_: Auto) -> Self {
        Self::Auto
    }
}

impl From<Stretch> for Units {
    fn from(stretch: Stretch) -> Self {
        Self::Stretch(stretch.0)
    }
}

impl From<Percentage> for Units {
    fn from(percentage: Percentage) -> Self {
        Self::Percentage(percentage.0)
    }
}

impl From<Length> for Units {
    fn from(length: Length) -> Self {
        if let Some(pixels) = length.to_px() {
            Self::Pixels(pixels)
        } else {
            Self::Auto
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    #[test]
    fn test_success() {
        assert_parse_value!(Units, "1px", Units::Pixels(1.0));
        assert_parse_value!(Units, "1cm", Units::Pixels(37.795277));
        assert_parse_value!(Units, "1mm", Units::Pixels(3.7795277));
        assert_parse_value!(Units, "1in", Units::Pixels(96.0));
        assert_parse_value!(Units, "1vmin", Units::Auto);
        assert_parse_value!(Units, "1rem", Units::Auto);
        assert_parse_value!(Units, "1ch", Units::Auto);

        assert_parse_value!(Units, "0%", Units::Percentage(0.0));
        assert_parse_value!(Units, "10%", Units::Percentage(0.1));
        assert_parse_value!(Units, "100%", Units::Percentage(1.0));

        assert_parse_value!(Units, "0st", Units::Stretch(0.0));
        assert_parse_value!(Units, "0.1st", Units::Stretch(0.1));
        assert_parse_value!(Units, "1st", Units::Stretch(1.0));

        assert_parse_value!(Units, "auto", Units::Auto);
    }

    #[test]
    fn test_failure() {
        assert_parse_value!(Units, "1abc");
        assert_parse_value!(Units, "px1");

        assert_parse_value!(Units, "%0");
        assert_parse_value!(Units, "10a");
        assert_parse_value!(Units, "%100");

        assert_parse_value!(Units, "0s");
        assert_parse_value!(Units, "1sta");
        assert_parse_value!(Units, "1s");

        assert_parse_value!(Units, "aauto");
        assert_parse_value!(Units, "1auto");
        assert_parse_value!(Units, "auto1");
    }
}
