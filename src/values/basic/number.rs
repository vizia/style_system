use crate::{impl_parse_expect, Parse};
use cssparser::Token;

impl_parse_expect! {
    i8,
    Token::Number { int_value: Some(int_value), .. } if i8::try_from(*int_value).is_ok() => *int_value as i8,
}
impl_parse_expect! {
    i16,
    Token::Number { int_value: Some(int_value), .. } if i16::try_from(*int_value).is_ok() => *int_value as i16,
}
impl_parse_expect! {
    i32,
    Token::Number { int_value: Some(int_value), .. } => *int_value,
}
impl_parse_expect! {
    i64,
    Token::Number { int_value: Some(int_value), .. } => i64::from(*int_value),
}
impl_parse_expect! {
    i128,
    Token::Number { int_value: Some(int_value), .. } => i128::from(*int_value),
}
impl_parse_expect! {
    isize,
    Token::Number { int_value: Some(int_value), .. } if isize::try_from(*int_value).is_ok() => *int_value as isize,
}

impl_parse_expect! {
    u8,
    Token::Number { int_value: Some(int_value), .. } if u8::try_from(*int_value).is_ok() => *int_value as u8,
}
impl_parse_expect! {
    u16,
    Token::Number { int_value: Some(int_value), .. } if u16::try_from(*int_value).is_ok() => *int_value as u16,
}
impl_parse_expect! {
    u32,
    Token::Number { int_value: Some(int_value), .. } if u32::try_from(*int_value).is_ok() => *int_value as u32,
}
impl_parse_expect! {
    u64,
    Token::Number { int_value: Some(int_value), .. } if u64::try_from(*int_value).is_ok() => *int_value as u64,
}
impl_parse_expect! {
    u128,
    Token::Number { int_value: Some(int_value), .. } if u128::try_from(*int_value).is_ok() => *int_value as u128,
}
impl_parse_expect! {
    usize,
    Token::Number { int_value: Some(int_value), .. } if usize::try_from(*int_value).is_ok() => *int_value as usize,
}

impl_parse_expect! {
    f32,
    Token::Number { value, .. } => *value,
}
impl_parse_expect! {
    f64,
    Token::Number { value, .. } => f64::from(*value),
}
