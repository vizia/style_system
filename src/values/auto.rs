use crate::{impl_parse_ident, Parse};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Auto;

impl_parse_ident! {
    Auto,
    "auto" => Auto,
}
