use crate::{macros::define_enum_value, Parse};

define_enum_value! {
    /// The shape the default view drawing algorithm should use for handling borders.
    pub enum BorderCornerShape {
        /// The round border corner shape.
        "round": Round,
        /// The bevel border corner shape.
        "bevel": Bevel,
    }
}

impl Default for BorderCornerShape {
    fn default() -> Self {
        BorderCornerShape::Round
    }
}
