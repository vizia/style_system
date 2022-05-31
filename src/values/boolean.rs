use crate::{macros::impl_parse_ident, Parse};

impl_parse_ident! {
    bool,
    "on" => true,
    "off" => false,
    "true" => true,
    "false" => false,
    "yes" => true,
    "no" => false,
}
