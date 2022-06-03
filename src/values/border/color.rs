use crate::{impl_from, macros::impl_parse, Color, Parse, Rect};

/// Defines the color of every border of a rectangle.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorderColor {
    /// The color of the top border.
    pub top: Color,
    /// The color of the right border.
    pub right: Color,
    /// The color of the bottom border.
    pub bottom: Color,
    /// The color of the left border.
    pub left: Color,
}

impl_parse! {
    BorderColor,

    try_parse {
        Rect<Color>,
    }
}

impl_from! {
    BorderColor,

    from {
        Rect<Color> => |x: Rect<Color>| BorderColor {
            top: x.0,
            right: x.1,
            bottom: x.2,
            left: x.3
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    macro_rules! border_color {
        (($($top: tt),+), ($($right: tt),+), ($($bottom: tt),+), ($($left: tt),+)) => {
            BorderColor {
                top: $crate::Color::RGBA($crate::RGBA::rgba($($top),+)),
                right: $crate::Color::RGBA($crate::RGBA::rgba($($right),+)),
                bottom: $crate::Color::RGBA($crate::RGBA::rgba($($bottom),+)),
                left: $crate::Color::RGBA($crate::RGBA::rgba($($left),+)),
            }
        };
    }

    assert_parse! {
        BorderColor, border_color,

        success {
            "#000000" => border_color!((0, 0, 0, 255), (0, 0, 0, 255), (0, 0, 0, 255), (0, 0, 0, 255)),
            "#FF00FF #00FF00" => border_color!((255, 0, 255, 255), (0, 255, 0, 255), (255, 0, 255, 255), (0, 255, 0, 255)),
            "#FF00FF #00FF00 #00FFFF" => border_color!((255, 0, 255, 255), (0, 255, 0, 255), (0, 255, 255, 255), (0, 255, 0, 255)),
            "#FF00FF #00FF00 #00FFFF #FFFFFF" => border_color!((255, 0, 255, 255), (0, 255, 0, 255), (0, 255, 255, 255), (255, 255, 255, 255)),
        }

        failure {
            "test",
            "123",
            "#000000 #000000 #000000 #000000 #000000",
        }
    }
}
