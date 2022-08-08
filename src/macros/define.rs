macro_rules! define_enum {
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
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        $vis enum $name {
            $(
                $(#[$meta])*
                $id,
            )+
        }

        $crate::impl_parse! {
            $name,

            tokens {
                ident {
                    $($str => $name::$id,)+
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            $crate::tests::assert_parse! {
                $name, assert_parse,

                ident {
                    $($str => $name::$id,)+
                }
            }
        }
  };
}

macro_rules! define_property {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident<'i> {
            $(
                $(#[$meta: meta])*
                $str: literal: $variant: ident($inner_ty: ty),
            )+
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq)]
        $vis enum $name<'i> {
            $(
                $(#[$meta])*
                $variant($inner_ty),
            )+
            Custom(CustomProperty<'i>),
        }

        impl<'i> $name<'i> {
            pub fn parse_value<'t>(name: cssparser::CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, CustomParseError<'i>>> {
                let location = input.current_source_location();
                let name_ref = name.as_ref();
                match name_ref {
                    $(
                        $str => Ok($name::$variant(<$inner_ty>::parse(input)?)),
                    )+
                    _ => {
                        if let Ok(custom) = CustomProperty::parse(name, input) {
                            Ok(Property::Custom(custom))
                        } else {
                            Err(cssparser::ParseError {
                                kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                                location,
                            })
                        }
                    }
                }
            }
        }
    };
}

pub(crate) use define_enum;
pub(crate) use define_property;
