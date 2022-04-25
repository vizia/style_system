macro_rules! enum_property {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $( $x: ident, )+
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq)]
        $vis enum $name {
            $(
                $x,
            )+
        }

        impl<'i> Parse<'i> for $name {
            fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
                let ident = input.expect_ident()?;
                match &ident[..] {
                    $(
                        s if s.eq_ignore_ascii_case(stringify!($x)) => Ok($name::$x),
                    )+
                    _ => return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
                }
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $(
                        s if s.eq_ignore_ascii_case(stringify!($x)) => Some($name::$x),
                    )+
                    _ => None
                }
            }
        }
    };
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $( $str: literal: $id: ident, )+
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq)]
        $vis enum $name {
            $(
                $id,
            )+
        }

        impl<'i> Parse<'i> for $name {
            fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError>> {
                let ident = input.expect_ident()?;
                match &ident[..] {
                    $(
                        s if s.eq_ignore_ascii_case($str) => Ok($name::$id),
                    )+
                    _ => return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
                }
            }
        }

        impl $name {
            #[allow(dead_code)]
            pub fn from_str(s: &str) -> Option<Self> {
            match s {
                $(
                s if s.eq_ignore_ascii_case($str) => Some($name::$id),
                )+
                _ => None
            }
            }
        }
    };
}

pub(crate) use enum_property;
