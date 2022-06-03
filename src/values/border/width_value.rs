use crate::{impl_from, impl_parse, BorderWidthKeyword, Length, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct BorderWidthValue(pub Length);

impl_parse! {
    BorderWidthValue,

    try_parse {
        BorderWidthKeyword,
        Length,
    }
}

impl_from! {
    BorderWidthValue,

    from {
        BorderWidthKeyword => |x| match x {
            BorderWidthKeyword::Thin => BorderWidthValue(Length::px(1.0)),
            BorderWidthKeyword::Medium => BorderWidthValue(Length::px(3.0)),
            BorderWidthKeyword::Thick => BorderWidthValue(Length::px(5.0)),
        },
        Length => |x| BorderWidthValue(x),
    }

    into {
        Length => |x: BorderWidthValue| x.0,
    }
}

impl Default for BorderWidthValue {
    fn default() -> Self {
        Self(Length::px(3.0))
    }
}
