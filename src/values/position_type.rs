use crate::{macros::define_enum_value, Parse};

define_enum_value! {
    /// The position type determines whether a node will be positioned in-line with its siblings or separate.
    pub enum PositionType {
        /// The node is positioned relative to the parent but ignores its siblings.
        "self-directed": SelfDirected,
        /// The node is positioned relative to the parent and in-line with its siblings.
        "parent-directed": ParentDirected,
    }
}
