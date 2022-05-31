use crate::{
    macros::{impl_from_newtype, impl_parse_expect},
    Parse,
};

pub struct Ident(pub String);

impl_parse_expect!(Ident, expect_ident, .as_ref().to_owned().into());
impl_from_newtype!(Ident => String);
