macro_rules! impl_parse_ident {
    (
        $name: ty,
        $(
            $pattern: expr => $result: expr,
        )+
    ) => {
        impl<'i> Parse<'i> for $name {
            fn parse<'t>(input: &mut cssparser::Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, $crate::CustomParseError<'i>>> {
                let location = input.current_source_location();
                let ident = input.expect_ident()?;

                Ok(match &ident[..] {
                $(
                    s if s.eq_ignore_ascii_case($pattern) => $result,
                )+
                    _ => return Err(location.new_unexpected_token_error(cssparser::Token::Ident(ident.clone()))),
                })
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::tests::assert_parse_idents;

            assert_parse_idents!($name, parse_ident, $($pattern => $result,)+);
        }
    };
}

macro_rules! impl_parse_try_parse {
    (
        $name: ty,
        $(
            $parse_type: ty,
        )+
    ) => {
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
    ($outer: tt ($inner: ty), $($other_newtype: ty,)*) => {
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
    ($name: ty, $($branches: tt)+) => {
        impl<'i> Parse<'i> for $name {
            fn parse<'t>(
                input: &mut cssparser::Parser<'i, 't>,
            ) -> Result<Self, cssparser::ParseError<'i, $crate::CustomParseError<'i>>> {
                let location = input.current_source_location();
                Ok(match input.next()? {
                    $($branches)+
                    _ => {
                        return Err(cssparser::ParseError {
                            kind: cssparser::ParseErrorKind::Custom($crate::CustomParseError::InvalidDeclaration),
                            location,
                        });
                    }
                })
            }
        }
    }
}

macro_rules! impl_parse_dimension {
    ($name: tt, $module: ident,
        $(
            $unit: literal => $ty: tt$(::$variant: tt)?$(($multiplier: expr))?,
        )+
    ) => {
        impl<'i> Parse<'i> for $name {
            fn parse<'t>(input: &mut cssparser::Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, $crate::CustomParseError<'i>>> {
                let location = input.current_source_location();
                let token = input.next()?;

                Ok(match &token {
                    cssparser::Token::Dimension { value, ref unit, .. } => {
                        match unit.as_ref() {
                            $(
                                s if s.eq_ignore_ascii_case($unit) => $ty$(::$variant)?(*value $(* $multiplier)?),
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
            use crate::tests::assert_parse_dimensions;

            assert_parse_dimensions! {
                $name, $module,
                $(
                    $unit => $ty$(::$variant)?$(($multiplier))?,
                )+
            }
        }
    };
}

pub(crate) use impl_from_newtype;
pub(crate) use impl_parse_dimension;
pub(crate) use impl_parse_expect;
pub(crate) use impl_parse_ident;
pub(crate) use impl_parse_try_parse;
