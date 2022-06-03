use crate::{impl_parse, BorderColor, BorderStyle, BorderWidth, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub width: BorderWidth,
    pub style: BorderStyle,
    pub color: BorderColor,
}

impl_parse! {
    Border,

    custom {
        |input| {
            Ok(Border {
                width: input.try_parse(BorderWidth::parse)?,
                style: input.try_parse(BorderStyle::parse)?,
                color: input.try_parse(BorderColor::parse)?,
            })
        }
    }
}
