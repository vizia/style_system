use crate::{impl_from, impl_parse, OverflowKeyword, Parse, Rect};

/// Determines how to deal with content that overflows the bounding box of the element on the x and y axis.
#[derive(Debug, Clone, PartialEq)]
pub struct Overflow {
    /// Determines how to deal with content that overflows the bounding box of the element on the x axis.
    pub x: OverflowKeyword,
    /// Determines how to deal with content that overflows the bounding box of the element on the y axis.
    pub y: OverflowKeyword,
}

impl_parse! {
    Overflow,

    try_parse {
        Rect<OverflowKeyword>,
    }
}

impl_from! {
    Overflow,

    from {
        Rect<OverflowKeyword> => |x: Rect<OverflowKeyword>| Overflow {
            x: x.0,
            y: x.1,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_parse, overflow};

    assert_parse! {
        Overflow, parse_value,

        custom {
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
}
