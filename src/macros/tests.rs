macro_rules! assert_parse {
    (
        $parse_type: ty, $module: ident,

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
    (
        $parse_type: ty, $module: ident,

        // Ident values
        $(
            ident {
                $(
                    $ident_string: expr => $ident_value: expr,
                )+
            }
        )?

        // Number values
        $(
            number {
                $number_type: expr,
            }
        )?

        // Percentage values
        $(
            percentage {
                $percentage_type: expr,
            }
        )?

        // Dimension values
        $(
            dimension {
                $(
                    $dimension_unit: expr =>
                        $dimension_type: tt$(::$dimension_variant: tt)?$(($dimension_multiplier: expr))?,
                )+
            }
        )?

        // Custom values
        $(
            custom {
                $(
                    success {
                        $(
                            $custom_success_string: expr => $custom_value: expr,
                        )+
                    }
                )?
                $(
                    failure {
                        $(
                            $custom_failure_string: expr,
                        )+
                    }
                )?
            }
        )?
    ) => {
        $crate::tests::assert_parse! {
            $parse_type, $module,

            success {
                // Ident values
                $(
                    $(
                        $ident_string => $ident_value,
                        &$ident_string.to_uppercase() => $ident_value,
                        &$ident_string.to_lowercase() => $ident_value,
                    )+
                )?

                // Number values
                $(
                    "1" => $number_type(1.0),
                    "2" => $number_type(2.0),
                    "3" => $number_type(3.0),
                    "4" => $number_type(4.0),
                    "5" => $number_type(5.0),
                    "6" => $number_type(6.0),
                    "7" => $number_type(7.0),
                    "8" => $number_type(8.0),
                    "9" => $number_type(9.0),
                    "0.1" => $number_type(0.1),
                    "0.2" => $number_type(0.2),
                    "0.3" => $number_type(0.3),
                    "0.4" => $number_type(0.4),
                    "0.5" => $number_type(0.5),
                    "0.6" => $number_type(0.6),
                    "0.7" => $number_type(0.7),
                    "0.8" => $number_type(0.8),
                    "0.9" => $number_type(0.9),
                    "1.0" => $number_type(1.0),
                    "0.00001" => $number_type(0.00001),
                    "0.99999" => $number_type(0.99999),
                )?

                // Percentage values
                $(
                    "10%" => $percentage_type(0.1),
                    "20%" => $percentage_type(0.2),
                    "30%" => $percentage_type(0.3),
                    "40%" => $percentage_type(0.4),
                    "50%" => $percentage_type(0.5),
                    "60%" => $percentage_type(0.6),
                    "70%" => $percentage_type(0.7),
                    "80%" => $percentage_type(0.8),
                    "90%" => $percentage_type(0.9),
                    "100%" => $percentage_type(1.0),
                    "0.001%" => $percentage_type(0.00001),
                    "99.999%" => $percentage_type(0.99999),
                )?

                // Dimension values
                $(
                    $(
                        concat!(1, $dimension_unit) => $dimension_type$(::$dimension_variant)?(1.0 $(* $dimension_multiplier)?),
                        concat!(2, $dimension_unit) => $dimension_type$(::$dimension_variant)?(2.0 $(* $dimension_multiplier)?),
                        concat!(3, $dimension_unit) => $dimension_type$(::$dimension_variant)?(3.0 $(* $dimension_multiplier)?),
                        concat!(4, $dimension_unit) => $dimension_type$(::$dimension_variant)?(4.0 $(* $dimension_multiplier)?),
                        concat!(5, $dimension_unit) => $dimension_type$(::$dimension_variant)?(5.0 $(* $dimension_multiplier)?),
                        concat!(6, $dimension_unit) => $dimension_type$(::$dimension_variant)?(6.0 $(* $dimension_multiplier)?),
                        concat!(7, $dimension_unit) => $dimension_type$(::$dimension_variant)?(7.0 $(* $dimension_multiplier)?),
                        concat!(8, $dimension_unit) => $dimension_type$(::$dimension_variant)?(8.0 $(* $dimension_multiplier)?),
                        concat!(9, $dimension_unit) => $dimension_type$(::$dimension_variant)?(9.0 $(* $dimension_multiplier)?),
                        concat!(10, $dimension_unit) => $dimension_type$(::$dimension_variant)?(10.0 $(* $dimension_multiplier)?),
                        concat!(0.00001, $dimension_unit) => $dimension_type$(::$dimension_variant)?(0.00001 $(* $dimension_multiplier)?),
                        concat!(99999.0, $dimension_unit) => $dimension_type$(::$dimension_variant)?(99999.0 $(* $dimension_multiplier)?),
                        &format!("{}{}", 1, $dimension_unit.to_uppercase()) => $dimension_type$(::$dimension_variant)?(1.0 $(* $dimension_multiplier)?),
                        &format!("{}{}", 1, $dimension_unit.to_lowercase()) => $dimension_type$(::$dimension_variant)?(1.0 $(* $dimension_multiplier)?),
                    )+
                )?

                // Custom values
                $(
                    $(
                        $(
                            $custom_success_string => $custom_value,
                        )+
                    )?
                )?
            }

            failure {
                // Ident values
                $(
                    $(
                        concat!($ident_string, "abc"),
                        concat!("abc", $ident_string),
                        concat!(1, $ident_string),
                        concat!($ident_string, 1),
                    )+
                )?

                // Number values
                $(
                    stringify!($number_type),
                    "abc",
                    "0abc",
                    "a0",
                )?

                // Percentage values
                $(
                    stringify!($percentage_type),
                    "abc",
                    "0abc",
                    "a0",
                )?

                // Dimension values
                $(
                    $(
                        "1",
                        concat!(1, $dimension_unit, "abc"),
                        concat!($dimension_unit, 1),
                    )+
                )?

                // Custom values
                $(
                    $(
                        $(
                            $custom_failure_string,
                        )+
                    )?
                )?
            }
        }
    }
}

macro_rules! border_color {
    (($($top: tt),+), ($($right: tt),+), ($($bottom: tt),+), ($($left: tt),+)) => {
        $crate::BorderColor {
            top: $crate::Color::RGBA($crate::RGBA::rgba($($top),+)),
            right: $crate::Color::RGBA($crate::RGBA::rgba($($right),+)),
            bottom: $crate::Color::RGBA($crate::RGBA::rgba($($bottom),+)),
            left: $crate::Color::RGBA($crate::RGBA::rgba($($left),+)),
        }
    };
}

macro_rules! border_width {
    ($top: expr, $right: expr, $bottom: expr, $left: expr) => {{
        use $crate::Length;

        $crate::BorderWidth {
            top: $crate::BorderWidthValue($top),
            right: $crate::BorderWidthValue($right),
            bottom: $crate::BorderWidthValue($bottom),
            left: $crate::BorderWidthValue($left),
        }
    }};
}

macro_rules! border_radius {
    ($top_left: expr, $top_right: expr, $bottom_right: expr, $bottom_left: expr) => {{
        use $crate::Length::*;
        use $crate::LengthValue::*;

        $crate::BorderRadius {
            top_left: Value($top_left),
            top_right: Value($top_right),
            bottom_right: Value($bottom_right),
            bottom_left: Value($bottom_left),
        }
    }};
}

macro_rules! border_style {
    ($top: ident, $right: ident, $bottom: ident, $left: ident) => {
        $crate::BorderStyle {
            top: $crate::BorderStyleKeyword::$top,
            right: $crate::BorderStyleKeyword::$right,
            bottom: $crate::BorderStyleKeyword::$bottom,
            left: $crate::BorderStyleKeyword::$left,
        }
    };
}

macro_rules! border {
    ($width: expr, $style: expr, $color: expr,) => {
        $crate::Border {
            width: $width,
            style: $style,
            color: $color,
        }
    };
}

macro_rules! box_shadow {
    (
        $x_offset: expr,
        $y_offset: expr,
        $blur: expr,
        $spread: expr,
        $color: expr,
        $inset: expr,
    ) => {
        $crate::BoxShadow {
            x_offset: $x_offset,
            y_offset: $y_offset,
            blur: $blur,
            spread: $spread,
            color: $color,
            inset: $inset,
        }
    };
}

macro_rules! color {
    ($red: literal, $green: literal, $blue: literal) => {
        $crate::Color::RGBA($crate::RGBA::rgb($red, $green, $blue))
    };
    ($red: literal, $green: literal, $blue: literal, $alpha: literal) => {
        $crate::Color::RGBA($crate::RGBA::rgba($red, $green, $blue, $alpha))
    };
}

macro_rules! overflow {
    ($x: expr, $y: expr) => {{
        use $crate::OverflowKeyword::*;

        $crate::Overflow { x: $x, y: $y }
    }};
}

macro_rules! transition {
    ($property: expr, $duration: expr, $delay: expr$(,)?) => {{
        Transition {
            property: $property,
            duration: $duration,
            delay: $delay,
        }
    }};
}

pub(crate) use assert_parse;
pub(crate) use border;
pub(crate) use border_color;
pub(crate) use border_radius;
pub(crate) use border_style;
pub(crate) use border_width;
pub(crate) use box_shadow;
pub(crate) use color;
pub(crate) use overflow;
pub(crate) use transition;
