use crate::{impl_from, macros::impl_parse, AutoKeyword, LengthPixels, Parse, Percentage, Stretch};
pub use morphorm::Units;

impl_parse! {
    Units,

    try_parse {
        AutoKeyword,
        Stretch,
        Percentage,
        LengthPixels,
    }
}

impl_from! {
    Units,

    from {
        AutoKeyword => |_| Self::Auto,
        Stretch => |x: Stretch| Self::Stretch(x.0),
        Percentage => |x: Percentage| Self::Percentage(x.0),
        LengthPixels => |x: LengthPixels| Self::Pixels(x.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tests::assert_parse, LengthValue};

    assert_parse! {
        Units, parse_units,

        ident {
            "auto" => Units::Auto,
        }

        percentage {
            Units::Percentage,
        }

        dimension {
            "px" => Units::Pixels,
            "in" => Units::Pixels(LengthValue::PX_PER_IN),
            "cm" => Units::Pixels(LengthValue::PX_PER_CM),
            "mm" => Units::Pixels(LengthValue::PX_PER_MM),
            "q" => Units::Pixels(LengthValue::PX_PER_Q),
            "pt" => Units::Pixels(LengthValue::PX_PER_PT),
            "pc" => Units::Pixels(LengthValue::PX_PER_PC),
            "st" => Units::Stretch,
        }
    }
}
