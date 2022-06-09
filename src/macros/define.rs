
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

        use crate::CustomProperty;

        #[derive(Debug, Clone, PartialEq)]
        $vis enum $name<'i> {
            $(
                $(#[$meta])*
                $variant($inner_ty),
            )+
            Custom(CustomProperty<'i>),
        }

        impl<'i> $name<'i> {
            pub fn parse_value<'t>(name: &str, input: &mut Parser<'i, 't>) -> Result<Self, cssparser::ParseError<'i, CustomParseError<'i>>> {
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
