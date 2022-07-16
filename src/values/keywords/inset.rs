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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        InsetKeyword, assert_inset_keyword,

        ident {
            "inset" => InsetKeyword,
        }
    }
}
