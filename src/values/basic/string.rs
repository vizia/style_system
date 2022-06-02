use crate::{macros::impl_parse_expect, Parse};
use cssparser::Token;

impl_parse_expect! {
    String,
    Token::QuotedString(ref value) => value.as_ref().to_owned(),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    assert_parse_value! {
        String, string,

        success {
            r#""test""# => String::from("test"),
            r#"'test'"# => String::from("test"),
            r#""abc"def"ghi""# => String::from("abc"),
            r#""a b c d e f g""# => String::from("a b c d e f g"),
        }

        failure {
            "test",
            "123",
        }
    }
}
