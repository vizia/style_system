use crate::{impl_from, macros::impl_parse, Length, Parse, Rect};

/// Defines the border radius of every corner of a rectangle.
#[derive(Debug, Clone, PartialEq)]
pub struct BorderRadius {
    /// The border radius of the top-left corner.
    pub top_left: Length,
    /// The border radius of the top-right corner.
    pub top_right: Length,
    /// The border radius of the bottom-right corner.
    pub bottom_right: Length,
    /// The border radius of the bottom-left corner.
    pub bottom_left: Length,
}

impl_parse! {
    BorderRadius,

    try_parse {
        Rect<Length>,
    }
}

impl_from! {
    BorderRadius,

    from {
        Rect<Length> => |x: Rect<Length>| BorderRadius {
            top_left: x.0,
            top_right: x.1,
            bottom_right: x.2,
            bottom_left: x.3
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    macro_rules! border_radius {
        ($top_left: expr, $top_right: expr, $bottom_right: expr, $bottom_left: expr) => {{
            use $crate::Length::*;
            use $crate::LengthValue::*;

            BorderRadius {
                top_left: Value($top_left),
                top_right: Value($top_right),
                bottom_right: Value($bottom_right),
                bottom_left: Value($bottom_left),
            }
        }};
    }

    assert_parse! {
        BorderRadius, border_radius,

        success {
            "10px" => border_radius!(Px(10.0), Px(10.0), Px(10.0), Px(10.0)),
            "10px 20px" => border_radius!(Px(10.0), Px(20.0), Px(10.0), Px(20.0)),
            "10px 20px 30px" => border_radius!(Px(10.0), Px(20.0), Px(30.0), Px(20.0)),
            "10px 20px 30px 40px" => border_radius!(Px(10.0), Px(20.0), Px(30.0), Px(40.0)),
        }

        failure {
            "px",
            "10px 20px 30px 40px 50px",
        }
    }
}
