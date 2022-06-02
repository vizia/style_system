use crate::{
    macros::{impl_from_newtype, impl_parse_try_parse},
    FontSizeKeyword, Parse,
};

/// A font size value.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FontSize(pub f32);

impl_parse_try_parse! {
    FontSize,
    FontSizeKeyword, f32,
}

impl_from_newtype! {
    FontSize(f32),
}

impl From<FontSizeKeyword> for FontSize {
    fn from(absolute_font_size: FontSizeKeyword) -> Self {
        match absolute_font_size {
            FontSizeKeyword::XXSmall => FontSize(8.0),
            FontSizeKeyword::XSmall => FontSize(10.0),
            FontSizeKeyword::Small => FontSize(12.0),
            FontSizeKeyword::Medium => FontSize(14.0),
            FontSizeKeyword::Large => FontSize(16.0),
            FontSizeKeyword::XLarge => FontSize(18.0),
            FontSizeKeyword::XXLarge => FontSize(20.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_parse_idents, assert_parse_numbers};

    assert_parse_numbers! {
        FontSize, font_size_numbers,
        FontSize,
    }

    assert_parse_idents! {
        FontSize, font_size_idents,

        "xx-small" => FontSize(8.0),
        "x-small" => FontSize(10.0),
        "small" => FontSize(12.0),
        "medium" => FontSize(14.0),
        "large" => FontSize(16.0),
        "x-large" => FontSize(18.0),
        "xx-large" => FontSize(20.0),
    }
}
