use crate::{impl_from, impl_parse, BorderStyleKeyword, Parse, Rect};

/// Defines the style of every border of a rectangle.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BorderStyle {
    /// The style of the top border.
    pub top: BorderStyleKeyword,
    /// The style of the right border.
    pub right: BorderStyleKeyword,
    /// The style of the bottom border.
    pub bottom: BorderStyleKeyword,
    /// The style of the left border.
    pub left: BorderStyleKeyword,
}

impl_parse! {
    BorderStyle,

    try_parse {
        Rect<BorderStyleKeyword>,
    }
}

impl_from! {
    BorderStyle,

    from {
        Rect<BorderStyleKeyword> => |x: Rect<BorderStyleKeyword>| BorderStyle {
            top: x.0,
            right: x.1,
            bottom: x.2,
            left: x.3
        },
    }
}
