use crate::{impl_from, impl_parse, traits::Parse, Percentage};

/// A value specifying the alpha channel or transparency of a color.
#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue(pub f32);

impl_parse! {
    AlphaValue,

    try_parse {
        Percentage,
        f32,
    }
}

impl_from! {
    AlphaValue,

    from {
        f32 => |x| AlphaValue(x),
        Percentage => |x: Percentage| AlphaValue(x.0),
    }

    into {
        f32 => |x: AlphaValue| x.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        AlphaValue, parse_numbers,

        number {
            AlphaValue,
        }

        percentage {
            AlphaValue,
        }
    }
}
