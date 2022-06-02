use crate::{
    macros::{impl_from_newtype, impl_parse_try_parse},
    percentage::Percentage,
    Parse,
};

/// An opacity value in the range of 0 to 1.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Opacity(pub f32);

impl_parse_try_parse! {
    Opacity,
    Percentage, f32,
}

impl_from_newtype! {
    Opacity(f32),
    Percentage,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_parse_numbers, assert_parse_percentages};

    assert_parse_percentages! {
        Opacity, parse_percentages,
        Opacity,
    }

    assert_parse_numbers! {
        Opacity, parse_numbers,
        Opacity,
    }
}
