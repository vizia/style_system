use crate::{define_enum_value, Parse};

define_enum_value! {
    pub enum HorizontalPositionKeyword {
        "left": Left,
        "right": Right,
    }
}
