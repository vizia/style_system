use crate::{impl_from, macros::define_enum_value, Parse};

define_enum_value! {
    /// Determines whether an entity will be rendered and acted on by the layout system.
    /// To make an entity invisible to rendering but still visible to layout, see [Visibility].
    pub enum Display {
        /// The entity will be rendered and acted on by the layout system.
        "flex": Flex,
        /// The entity will not be rendered and acted on by the layout system.
        "none": None,
    }
}

impl_from! {
    Display,

    from {
        bool => |x| if x { Display::Flex } else { Display::None },
    }
}

impl Default for Display {
    fn default() -> Self {
        Display::Flex
    }
}
