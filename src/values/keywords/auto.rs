use crate::{impl_parse, Parse};

/// The 'auto' keyword.
///
/// It is used to parse the [`Units::Auto`](crate::Units::Auto) variant.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AutoKeyword;

impl_parse! {
    AutoKeyword,

    tokens {
        ident {
            "auto" => AutoKeyword,
        }
    }
}
