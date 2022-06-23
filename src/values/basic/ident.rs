use crate::{macros::impl_parse, Parse};

/// A simple ident.
#[derive(Debug, Clone, PartialEq)]
pub struct Ident(pub String);

impl_parse! {
    Ident,

    tokens {
        custom {
            cssparser::Token::Ident(ident) => ident.as_ref().to_owned().into(),
        }
    }
}

impl From<String> for Ident {
    fn from(string: String) -> Self {
        Ident(string)
    }
}

impl From<Ident> for String {
    fn from(ident: Ident) -> Self {
        ident.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        Ident, assert_ident,

        custom {
            success {
                "ident" => Ident(String::from("ident")),
                "border" => Ident(String::from("border")),
                "color" => Ident(String::from("color")),
                "yes" => Ident(String::from("yes")),
                "no" => Ident(String::from("no")),
            }

            failure {
                "123",
                "123ident",
            }
        }
    }
}
