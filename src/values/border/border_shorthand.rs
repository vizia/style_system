use crate::{impl_parse, BorderColor, BorderStyle, BorderWidth, CustomParseError, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub width: Option<BorderWidth>,
    pub style: Option<BorderStyle>,
    pub color: Option<BorderColor>,
}

impl_parse! {
    Border,

    custom {
        |input| {
            let location = input.current_source_location();
            let width = input.try_parse(BorderWidth::parse).map(|x| Some(x)).unwrap_or(None);
            let style = input.try_parse(BorderStyle::parse).map(|x| Some(x)).unwrap_or(None);
            let color = input.try_parse(BorderColor::parse).map(|x| Some(x)).unwrap_or(None);

            if (width.is_some() || style.is_some() || color.is_some()) && input.is_exhausted() {
                Ok(Border { width, style, color })
            } else {
                return Err(cssparser::ParseError {
                    kind: cssparser::ParseErrorKind::Custom(CustomParseError::InvalidDeclaration),
                    location,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_parse, border, border_color, border_style, border_width};

    assert_parse! {
        Border, assert_border,

        custom {
            success {
                "thin thick" => border!(
                    Some(border_width!(Length::px(1.0), Length::px(5.0), Length::px(1.0), Length::px(5.0))),
                    None,
                    None,
                ),
                "solid dotted" => border!(
                    None,
                    Some(border_style!(Solid, Dotted, Solid, Dotted)),
                    None,
                ),
                "#00FF00 #FF00FF" => border!(
                    None,
                    None,
                    Some(border_color!((0, 255, 0, 255), (255, 0, 255, 255), (0, 255, 0, 255), (255, 0, 255, 255))),
                ),
                "thin thick solid dotted #00FF00 #FF00FF" => border!(
                    Some(border_width!(Length::px(1.0), Length::px(5.0), Length::px(1.0), Length::px(5.0))),
                    Some(border_style!(Solid, Dotted, Solid, Dotted)),
                    Some(border_color!((0, 255, 0, 255), (255, 0, 255, 255), (0, 255, 0, 255), (255, 0, 255, 255))),
                ),
            }

            failure {
                "test",
                "123",
                "test #00FF00",
                "#00FF00 test",
                "thin solid #00FF00 x",
            }
        }
    }
}
