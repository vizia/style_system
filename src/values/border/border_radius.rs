use crate::{macros::impl_parse, Length, Parse, Rect};

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

impl BorderRadius {
    pub fn new(
        top_left: Length,
        top_right: Length,
        bottom_right: Length,
        bottom_left: Length,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

impl_parse! {
    BorderRadius,

    try_parse {
        Rect<Length>,
    }
}

impl From<Rect<Length>> for BorderRadius {
    fn from(rect: Rect<Length>) -> Self {
        BorderRadius::new(rect.0, rect.1, rect.2, rect.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;

    assert_parse! {
        BorderRadius, assert_border_radius,

        success {
            "10px" => BorderRadius::new(Length::px(10.0), Length::px(10.0), Length::px(10.0), Length::px(10.0)),
            "10px 20px" =>  BorderRadius::new(Length::px(10.0), Length::px(20.0), Length::px(10.0), Length::px(20.0)),
            "10px 20px 30px" =>  BorderRadius::new(Length::px(10.0), Length::px(20.0), Length::px(30.0), Length::px(20.0)),
            "10px 20px 30px 40px" =>  BorderRadius::new(Length::px(10.0), Length::px(20.0), Length::px(30.0), Length::px(40.0)),
        }

        failure {
            "px",
            "10px 20px 30px 40px 50px",
        }
    }
}
