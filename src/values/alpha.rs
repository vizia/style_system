use crate::{impl_from_newtype, impl_parse_try_parse, traits::Parse, Percentage};

#[derive(Debug, Clone, PartialEq)]
pub struct AlphaValue(pub f32);

impl_parse_try_parse!(AlphaValue => Percentage, f32);
impl_from_newtype!(AlphaValue => f32, Percentage);
