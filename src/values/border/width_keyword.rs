use crate::{define_enum_value, Parse};

define_enum_value! {
    pub enum BorderWidthKeyword {
        "thin": Thin,
        "medium": Medium,
        "thick": Thick,
    }
}
