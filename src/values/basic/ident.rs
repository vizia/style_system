use crate::{
    macros::{impl_from, impl_parse},
    Parse,
};

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

impl_from! {
    Ident,

    from {
        String => |x| Ident(x),
    }

    into {
        String => |x: Ident| x.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
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
