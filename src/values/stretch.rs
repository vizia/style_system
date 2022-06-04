use crate::{impl_parse, macros::impl_from, Parse};

/// A factor of the remaining free space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stretch(pub f32);

impl_parse! {
    Stretch,

    tokens {
        dimension {
            "st" => Stretch,
        }
    }
}

impl_from! {
    Stretch,

    from {
        f32 => |x| Stretch(x),
    }

    into {
        f32 => |x: Stretch| x.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Stretch, assert_stretch,

        dimension {
            "st" => Stretch,
        }
    }
}
