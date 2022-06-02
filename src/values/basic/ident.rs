use crate::{
    macros::{impl_from_newtype, impl_parse_expect},
    Parse,
};

/// A simple ident stored.
#[derive(Debug, Clone, PartialEq)]
pub struct Ident(pub String);

impl_parse_expect! {
    Ident,
    cssparser::Token::Ident(ident) => ident.as_ref().to_owned().into(),
}

impl_from_newtype! {
    Ident(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    assert_parse_value! {
        Ident, ident,

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
