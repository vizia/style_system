use crate::{
    macros::{impl_from, impl_parse},
    Parse, Percentage,
};

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

impl_from! {
    Opacity,

    from {
        f32 => |x| Opacity(x),
        Percentage => |x: Percentage| Opacity(x.0),
    }

    into {
        f32 => |x: Opacity| x.0,
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
