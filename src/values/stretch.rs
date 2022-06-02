use crate::{
    macros::{impl_from_newtype, impl_parse_dimension},
    Parse,
};

/// A factor of the remaining free space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stretch(pub f32);

impl_parse_dimension! {
    Stretch, stretch,

    "st" => Stretch,
}

impl_from_newtype! {
    Stretch(f32),
}
