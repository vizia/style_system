use crate::{impl_from_newtype, impl_parse_try_parse, traits::Parse, Percentage};

/// A value specifying the alpha channel or transparency of a color.
#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue(pub f32);

impl_parse_try_parse! {
    AlphaValue,
    Percentage, f32,
}

impl_from_newtype! {
    AlphaValue(f32),
    Percentage,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_parse_numbers, assert_parse_percentages};

    assert_parse_numbers! {
        AlphaValue, parse_numbers,
        AlphaValue,
    }

    assert_parse_percentages! {
        AlphaValue, parse_percentages,
        AlphaValue,
    }
}
