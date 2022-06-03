use crate::{impl_from, macros::define_enum_value, Parse};

define_enum_value! {
    /// Determines whether an entity will be rendered.
    ///
    /// An invisible entity will still be acted upon by the layout system.
    /// Use [`Display`](crate::values::Display) to hide an entity from both rendering and layout.
    pub enum Visibility {
        /// The entity will be rendered.
        "visible": Visible,
        /// The entity will not rendered, but will still be acted upon by the layout system.
        "invisible": Invisible,
    }
}

impl_from! {
    Visibility,

    from {
        bool => |x| if x { Visibility::Visible } else { Visibility::Invisible },
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}
