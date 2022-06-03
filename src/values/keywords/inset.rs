use crate::{impl_parse, Parse};

/// The 'inset' keyword.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InsetKeyword;

impl_parse! {
    InsetKeyword,

    tokens {
        ident {
            "inset" => InsetKeyword,
        }
    }
}
