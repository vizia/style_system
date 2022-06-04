use crate::{impl_from, impl_parse, BorderWidthValue, Parse, Rect};

/// Defines the width of every border of a rectangle.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorderWidth {
    /// The width of the top border.
    pub top: BorderWidthValue,
    /// The width of the right border.
    pub right: BorderWidthValue,
    /// The width of the bottom border.
    pub bottom: BorderWidthValue,
    /// The width of the left border.
    pub left: BorderWidthValue,
}

impl_parse! {
    BorderWidth,

    try_parse {
        Rect<BorderWidthValue>,
    }
}

impl_from! {
    BorderWidth,

    from {
        Rect<BorderWidthValue> => |x: Rect<BorderWidthValue>| BorderWidth {
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
    use crate::tests::{assert_parse, border_width};

    assert_parse! {
        BorderWidth, assert_border_width,

        custom {
            success {
                "10px" => border_width!(Length::px(10.0), Length::px(10.0), Length::px(10.0), Length::px(10.0)),
                "10px 20px" => border_width!(Length::px(10.0), Length::px(20.0), Length::px(10.0), Length::px(20.0)),
                "10px 20px 30px" => border_width!(Length::px(10.0), Length::px(20.0), Length::px(30.0), Length::px(20.0)),
                "10px 20px 30px 40px" => border_width!(Length::px(10.0), Length::px(20.0), Length::px(30.0), Length::px(40.0)),

                "thin" => border_width!(Length::px(1.0), Length::px(1.0), Length::px(1.0), Length::px(1.0)),
                "thin medium" => border_width!(Length::px(1.0), Length::px(3.0), Length::px(1.0), Length::px(3.0)),
                "thin medium thick" => border_width!(Length::px(1.0), Length::px(3.0), Length::px(5.0), Length::px(3.0)),
                "thin medium thick thin" => border_width!(Length::px(1.0), Length::px(3.0), Length::px(5.0), Length::px(1.0)),
            }

            failure {
                "test",
                "123",
                "10px 20px 30px 40px 50px",
            }
        }
    }
}
