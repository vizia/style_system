use crate::{
    macros::{impl_from_newtype, impl_parse_dimension},
    Parse,
};

/// A duration value normalized to seconds.
///
/// The supported time units are seconds (`s`) and milliseconds (`ms`).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Duration(pub f32);

impl_parse_dimension! {
    Duration,
    "s" => Duration,
    "ms" => Duration(0.001),
}

impl_from_newtype!(Duration => f32);
