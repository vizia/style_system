use crate::{macros::define_enum_value, Parse};

define_enum_value! {
    /// A font size value parsed from a font size name like "medium".
    pub enum AbsoluteFontSize {
        "xx-small": XXSmall,
        "x-small": XSmall,
        "small": Small,
        "medium": Medium,
        "large": Large,
        "x-large": XLarge,
        "xx-large": XXLarge,
    }
}
