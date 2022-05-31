macro_rules! define_enum_value {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$meta: meta])*
                $str: literal: $id: ident,
            )+
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Copy, Clone, PartialEq)]
        $vis enum $name {
            $(
                $(#[$meta])*
                $id,
            )+
        }

        $crate::impl_parse_ident!($name, $($str => $name::$id,)+);
  };
}

macro_rules! impl_parse_ident {
  ($name: tt, $($str: literal => $result: expr,)+) => {
    impl<'i> Parse<'i> for $name {
      fn parse<'t>(input: &mut cssparser::Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, $crate::CustomParseError<'i>>> {
        let location = input.current_source_location();
        let ident = input.expect_ident()?;

        Ok(match &ident[..] {
          $(
            s if s.eq_ignore_ascii_case($str) => $result,
          )+
            _ => return Err(location.new_unexpected_token_error(cssparser::Token::Ident(ident.clone()))),
        })
      }
    }

    #[cfg(test)]
    mod tests {
      use super::*;
      use crate::tests::assert_parse_value;

      #[test]
      fn test_success() {
        $(
          assert_parse_value!($name, $str, $result);
          assert_parse_value!($name, &$str.to_uppercase(), $result);
          assert_parse_value!($name, &$str.to_lowercase(), $result);
        )+
      }

      #[test]
      fn test_failure() {
        $(
          assert_parse_value!($name, concat!($str, "abc"));
          assert_parse_value!($name, concat!("abc", $str));
          assert_parse_value!($name, concat!(1, $str));
          assert_parse_value!($name, concat!($str, 1));
        )+
      }
    }
  };
}

#[cfg(test)]
pub mod tests {
    macro_rules! assert_parse_value {
        ($value_type:ty, $string:expr, $value:expr) => {{
            let string = $string;
            let mut parser_input = cssparser::ParserInput::new(string);
            let mut parser = cssparser::Parser::new(&mut parser_input);
            let result = <$value_type>::parse(&mut parser);
            assert_eq!(result, Ok($value));
        }};
        ($value_type:ty, $string:expr) => {{
            let mut parser_input = cssparser::ParserInput::new($string);
            let mut parser = cssparser::Parser::new(&mut parser_input);
            let result = <$value_type>::parse(&mut parser);
            assert!(result.is_err());
        }};
    }

    pub(crate) use assert_parse_value;
}

macro_rules! impl_parse_try_parse {
  ($name: tt => $($parse_type: ty),+) => {
    impl<'i> Parse<'i> for $name {
      fn parse<'t>(input: &mut cssparser::Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, $crate::CustomParseError<'i>>> {
        let location = input.current_source_location();

        $(
          if let Ok(x) = input.try_parse(<$parse_type>::parse) {
            return Ok(x.into());
          }
        )+

        Err(cssparser::ParseError {
            kind: cssparser::ParseErrorKind::Custom($crate::CustomParseError::InvalidDeclaration),
            location,
        })
      }
    }
  };
}

macro_rules! impl_from_newtype {
    ($outer: ty => $inner: ty $(, $other_newtype: ty)*) => {
        impl From<$inner> for $outer {
            fn from(x: $inner) -> Self {
                Self(x)
            }
        }

        impl From<$outer> for $inner {
            fn from(x: $outer) -> Self {
                x.0
            }
        }

        $(
          impl From<$other_newtype> for $outer {
            fn from(x: $other_newtype) -> Self {
              Self(x.0)
            }
          }
        )*
    };
}

macro_rules! impl_parse_expect {
    ($name: ty, $expect: ident, $($cast: tt)*) => {
        impl<'i> Parse<'i> for $name {
            fn parse<'t>(
                input: &mut cssparser::Parser<'i, 't>,
            ) -> Result<Self, cssparser::ParseError<'i, $crate::CustomParseError<'i>>> {
                let x = input.$expect()? $($cast)*;
                Ok(x)
            }
        }
    };
}

macro_rules! impl_parse_dimension {
  ($name: tt, $($unit: literal => $ty: tt$(::$variant: tt)?$(($mutliplier: literal))?,)+) => {
    impl<'i> Parse<'i> for $name {
      fn parse<'t>(input: &mut cssparser::Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, $crate::CustomParseError<'i>>> {
        let location = input.current_source_location();
        let token = input.next()?;

        Ok(match &token {
          cssparser::Token::Dimension { value, ref unit, .. } => {
            match unit.as_ref() {
              $(
                s if s.eq_ignore_ascii_case($unit) => $ty$(::$variant)?(*value $(* $mutliplier)?),
              )*
              _ => return Err(location.new_unexpected_token_error(token.clone())),
            }
          }
          _ => return Err(location.new_unexpected_token_error(token.clone())),
        })
      }
    }

    #[cfg(test)]
    mod tests {
      use super::*;
      use crate::tests::assert_parse_value;

      #[test]
      fn test_success() {
        $(
          assert_parse_value!($name, concat!(1, $unit), $ty$(::$variant)?(1.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(2, $unit), $ty$(::$variant)?(2.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(3, $unit), $ty$(::$variant)?(3.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(4, $unit), $ty$(::$variant)?(4.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(5, $unit), $ty$(::$variant)?(5.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(6, $unit), $ty$(::$variant)?(6.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(7, $unit), $ty$(::$variant)?(7.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(8, $unit), $ty$(::$variant)?(8.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(9, $unit), $ty$(::$variant)?(9.0 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(10, $unit), $ty$(::$variant)?(10.0 $(* $mutliplier)?));

          assert_parse_value!($name, concat!(0.00001, $unit), $ty$(::$variant)?(0.00001 $(* $mutliplier)?));
          assert_parse_value!($name, concat!(99999.0, $unit), $ty$(::$variant)?(99999.0 $(* $mutliplier)?));
          assert_parse_value!($name, &format!("{}{}", 1, $unit.to_uppercase()), $ty$(::$variant)?(1.0 $(* $mutliplier)?));
        )+
      }

      #[test]
      fn test_failure() {
        $(
          assert_parse_value!($name, "1");
          assert_parse_value!($name, $unit);
          assert_parse_value!($name, concat!(1, $unit, "abc"));
          assert_parse_value!($name, concat!($unit, 1));
        )+
      }
    }
  };
}

macro_rules! define_property {
  (
    $(#[$outer:meta])*
      $vis:vis enum $name:ident {
        $(
          $(#[$meta: meta])*
            $str: literal: $variant: ident($inner_ty: ty),
        )+
      }
  ) => {
    $(#[$outer])*
    #[derive(Debug, Clone, PartialEq)]
    $vis enum $name {
      $(
        $(#[$meta])*
          $variant($inner_ty),
      )+
    }

    impl $name {
      pub fn parse_value<'i, 't>(name: &str, input: &mut Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, CustomParseError<'i>>> {
        let location = input.current_source_location();
        match name {
          $(
            $str => Ok($name::$variant(<$inner_ty>::parse(input)?)),
          )+
          _ => {
            return Err(cssparser::ParseError {
              kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
              location,
            });
          }
        }
      }
    }
  };
}

pub(crate) use define_enum_value;
pub(crate) use define_property;
pub(crate) use impl_from_newtype;
pub(crate) use impl_parse_dimension;
pub(crate) use impl_parse_expect;
pub(crate) use impl_parse_ident;
pub(crate) use impl_parse_try_parse;
