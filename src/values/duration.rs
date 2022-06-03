use crate::{
    macros::{impl_from, impl_parse},
    Parse,
};

/// A duration value normalized to seconds.
///
/// The supported time units are seconds (`s`) and milliseconds (`ms`).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Duration(pub f32);

impl_parse! {
    Duration,

    tokens {
        dimension {
            "s" => Duration,
            "ms" => Duration(0.001),
        }
    }
}

impl_from! {
    Duration,

    from {
        f32 => |x| Duration(x),
    }

    into {
        f32 => |x: Duration| x.0,
    }
}
