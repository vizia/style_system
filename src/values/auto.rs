use crate::{impl_parse_ident, Parse};

/// The 'auto' keyword.
///
/// It is used to parse the [`Units::Auto`](crate::Units::Auto) variant.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AutoKeyword;

impl_parse_ident! {
    AutoKeyword,

    "auto" => AutoKeyword,
}
