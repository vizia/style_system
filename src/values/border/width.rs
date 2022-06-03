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
