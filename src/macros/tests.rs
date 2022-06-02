macro_rules! assert_parse_value {
    ($parse_type: ty, $module: ident,
     $(
         success {
             $(
                 $success_string: expr => $value: expr,
             )+
         }
     )?
     $(
         failure {
             $(
                 $failure_string: expr,
             )+
         }
     )?
    ) => {
        mod $module {
            use super::*;

            $(
                #[test]
                fn test_success() {
                    $({
                        let success_string = $success_string;
                        let mut parser_input = cssparser::ParserInput::new(success_string);
                        let mut parser = cssparser::Parser::new(&mut parser_input);
                        let result = <$parse_type>::parse(&mut parser);
                        assert_eq!(result, Ok($value));
                    })+
                }
            )?

            $(
                #[test]
                fn test_failure() {
                    $({
                        let failure_string = $failure_string;
                        let mut parser_input = cssparser::ParserInput::new(failure_string);
                        let mut parser = cssparser::Parser::new(&mut parser_input);
                        let result = <$parse_type>::parse(&mut parser);
                        assert!(result.is_err());
                    })+
                }
            )?
        }
    };
}

macro_rules! assert_parse_numbers {
    (
        $parse_type: ty, $module: ident,
        $result_type: expr,
    ) => {
        $crate::tests::assert_parse_value! {
            $parse_type, $module,

            success {
                "1" => $result_type(1.0),
                "2" => $result_type(2.0),
                "3" => $result_type(3.0),
                "4" => $result_type(4.0),
                "5" => $result_type(5.0),
                "6" => $result_type(6.0),
                "7" => $result_type(7.0),
                "8" => $result_type(8.0),
                "9" => $result_type(9.0),
                "0.1" => $result_type(0.1),
                "0.2" => $result_type(0.2),
                "0.3" => $result_type(0.3),
                "0.4" => $result_type(0.4),
                "0.5" => $result_type(0.5),
                "0.6" => $result_type(0.6),
                "0.7" => $result_type(0.7),
                "0.8" => $result_type(0.8),
                "0.9" => $result_type(0.9),
                "1.0" => $result_type(1.0),
                "0.00001" => $result_type(0.00001),
                "0.99999" => $result_type(0.99999),
            }

            failure {
                "abc",
                "0abc",
                "a0",
            }
        }
    };
}

macro_rules! assert_parse_percentages {
    (
        $parse_type: ty, $module: ident,
        $result_type: expr,
    ) => {
        $crate::tests::assert_parse_value! {
            $parse_type, $module,

            success {
                "10%" => $result_type(0.1),
                "20%" => $result_type(0.2),
                "30%" => $result_type(0.3),
                "40%" => $result_type(0.4),
                "50%" => $result_type(0.5),
                "60%" => $result_type(0.6),
                "70%" => $result_type(0.7),
                "80%" => $result_type(0.8),
                "90%" => $result_type(0.9),
                "100%" => $result_type(1.0),
                "0.001%" => $result_type(0.00001),
                "99.999%" => $result_type(0.99999),
            }

            failure  {
                "abc",
            }
        }
    };
}

macro_rules! assert_parse_idents {
    (
        $parse_type: ty, $module: ident,
        $(
            $string: literal => $result: expr,
        )+
    ) => {
        $crate::tests::assert_parse_value! {
            $parse_type, $module,

            success {
                $(
                    $string => $result,
                    &$string.to_uppercase() => $result,
                    &$string.to_lowercase() => $result,
                )+
            }

            failure {
                $(
                    concat!($string, "abc"),
                    concat!("abc", $string),
                    concat!(1, $string),
                    concat!($string, 1),
                )+
            }
        }
    };
}

macro_rules! assert_parse_dimensions {
    (
        $parse_type: ty, $module: ident,
        $(
            $unit: literal => $ty: tt$(::$variant: tt)?$(($multiplier: expr))?,
        )+
    ) => {
        $crate::tests::assert_parse_value! {
            $parse_type, $module,

            success {
                $(
                    concat!(1, $unit) => $ty$(::$variant)?(1.0 $(* $multiplier)?),
                    concat!(2, $unit) => $ty$(::$variant)?(2.0 $(* $multiplier)?),
                    concat!(3, $unit) => $ty$(::$variant)?(3.0 $(* $multiplier)?),
                    concat!(4, $unit) => $ty$(::$variant)?(4.0 $(* $multiplier)?),
                    concat!(5, $unit) => $ty$(::$variant)?(5.0 $(* $multiplier)?),
                    concat!(6, $unit) => $ty$(::$variant)?(6.0 $(* $multiplier)?),
                    concat!(7, $unit) => $ty$(::$variant)?(7.0 $(* $multiplier)?),
                    concat!(8, $unit) => $ty$(::$variant)?(8.0 $(* $multiplier)?),
                    concat!(9, $unit) => $ty$(::$variant)?(9.0 $(* $multiplier)?),
                    concat!(10, $unit) => $ty$(::$variant)?(10.0 $(* $multiplier)?),
                    concat!(0.00001, $unit) => $ty$(::$variant)?(0.00001 $(* $multiplier)?),
                    concat!(99999.0, $unit) => $ty$(::$variant)?(99999.0 $(* $multiplier)?),
                    &format!("{}{}", 1, $unit.to_uppercase()) => $ty$(::$variant)?(1.0 $(* $multiplier)?),
                    &format!("{}{}", 1, $unit.to_lowercase()) => $ty$(::$variant)?(1.0 $(* $multiplier)?),
                )+
            }

            failure {
                $(
                    "1",
                    concat!(1, $unit, "abc"),
                    concat!($unit, 1),
                )+
            }
        }
    };
}

pub(crate) use assert_parse_dimensions;
pub(crate) use assert_parse_idents;
pub(crate) use assert_parse_numbers;
pub(crate) use assert_parse_percentages;
pub(crate) use assert_parse_value;
