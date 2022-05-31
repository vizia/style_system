use crate::{define_enum_value, Parse};

define_enum_value! {
    pub enum VerticalPositionKeyword {
        "top": Top,
        "bottom": Bottom,
    }
}
