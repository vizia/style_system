use crate::{macros::impl_parse_expect, Parse};

impl_parse_expect!(String, expect_string, .as_ref().to_owned());

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    #[test]
    fn test_success() {
        assert_parse_value!(String, r#""test""#, String::from("test"));
        assert_parse_value!(String, r#"'test'"#, String::from("test"));
        assert_parse_value!(String, r#""abc"def"ghi""#, String::from("abc"));
        assert_parse_value!(String, r#""a b c d e f g""#, String::from("a b c d e f g"));
    }

    #[test]
    fn test_failure() {
        assert_parse_value!(String, "test");
        assert_parse_value!(String, "123");
    }
}
