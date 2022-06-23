use crate::{macros::impl_parse, Parse, Percentage};

/// An opacity value in the range of 0 to 1.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Opacity(pub f32);

impl_parse! {
    Opacity,

    try_parse {
        Percentage,
        f32,
    }
}

impl From<f32> for Opacity {
    fn from(number: f32) -> Self {
        Opacity(number)
    }
}

impl From<Percentage> for Opacity {
    fn from(percentage: Percentage) -> Self {
        Opacity(percentage.0)
    }
}

impl From<Opacity> for f32 {
    fn from(opacity: Opacity) -> Self {
        opacity.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Opacity, opacity,

        number {
            Opacity,
        }

        percentage {
            Opacity,
        }
    }
}
