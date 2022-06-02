use crate::{impl_parse_try_parse, OverflowKeyword, Parse, Rect};

/// Determines how to deal with content that overflows the bounding box of the element on the x and y axis.
#[derive(Debug, Clone, PartialEq)]
pub struct Overflow {
    /// Determines how to deal with content that overflows the bounding box of the element on the x axis.
    pub x: OverflowKeyword,
    /// Determines how to deal with content that overflows the bounding box of the element on the y axis.
    pub y: OverflowKeyword,
}

impl_parse_try_parse! {
    Overflow,
    Rect<OverflowKeyword>,
}

impl From<Rect<OverflowKeyword>> for Overflow {
    fn from(rect: Rect<OverflowKeyword>) -> Self {
        Overflow {
            x: rect.0,
            y: rect.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OverflowKeyword::*, *};
    use crate::tests::assert_parse_value;

    macro_rules! overflow {
        ($x: expr, $y: expr) => {
            Overflow { x: $x, y: $y }
        };
    }

    assert_parse_value! {
        Overflow, parse_value,

        success {
            "visible" => overflow!(Visible, Visible),
            "hidden" => overflow!(Hidden, Hidden),
            "clip" => overflow!(Clip, Clip),
            "scroll" => overflow!(Scroll, Scroll),
            "visible visible" => overflow!(Visible, Visible),
            "hidden hidden" => overflow!(Hidden, Hidden),
            "clip clip" => overflow!(Clip, Clip),
            "scroll scroll" => overflow!(Scroll, Scroll),
            "visible hidden" => overflow!(Visible, Hidden),
            "hidden clip" => overflow!(Hidden, Clip),
            "clip scroll" => overflow!(Clip, Scroll),
            "scroll visible" => overflow!(Scroll, Visible),
        }

        failure {
            "test",
            "123",
            "test visible",
        }
    }
}
