use crate::{
    auto::AutoKeyword, length::Length, macros::impl_parse_try_parse, percentage::Percentage,
    stretch::Stretch, Parse,
};
pub use morphorm::Units;

impl_parse_try_parse! {
    Units,
    AutoKeyword, Stretch, Percentage, Length,
}

impl From<AutoKeyword> for Units {
    fn from(_: AutoKeyword) -> Self {
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
    use crate::{
        tests::{assert_parse_dimensions, assert_parse_idents, assert_parse_percentages},
        LengthValue,
    };

    assert_parse_percentages! {
        Units, parse_percentages,
        Units::Percentage,
    }

    assert_parse_dimensions! {
        Units, parse_dimensions,

        "px" => Units::Pixels,
        "in" => Units::Pixels(LengthValue::PX_PER_IN),
        "cm" => Units::Pixels(LengthValue::PX_PER_CM),
        "mm" => Units::Pixels(LengthValue::PX_PER_MM),
        "q" => Units::Pixels(LengthValue::PX_PER_Q),
        "pt" => Units::Pixels(LengthValue::PX_PER_PT),
        "pc" => Units::Pixels(LengthValue::PX_PER_PC),
        "st" => Units::Stretch,
    }

    assert_parse_idents! {
        Units, parse_idents,

        "auto" => Units::Auto,
    }
}
