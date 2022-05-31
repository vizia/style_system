use crate::{define_enum_value, Parse};

define_enum_value! {
    pub enum TextOverflow {
        "clip": Clip,
        "ellipsis": Ellipsis,
    }
}
