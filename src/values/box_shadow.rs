use crate::{impl_parse, Color, InsetKeyword, Length, Parse};

#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadow {
    pub x_offset: Length,
    pub y_offset: Length,
    pub blur: Option<Length>,
    pub spread: Option<Length>,
    pub color: Option<Color>,
    pub inset: bool,
}

impl_parse! {
    BoxShadow,

    custom {
        |input| {
            let x_offset = Length::parse(input)?;
            let y_offset = Length::parse(input)?;
            let blur = input
                .try_parse(Length::parse)
                .map(|blur| Some(blur))
                .unwrap_or(None);
            let spread = input
                .try_parse(Length::parse)
                .map(|spread| Some(spread))
                .unwrap_or(None);
            let color = input
                .try_parse(Color::parse)
                .map(|color| Some(color))
                .unwrap_or(None);
            let inset = input
                .try_parse(InsetKeyword::parse)
                .map(|_| true)
                .unwrap_or(false);

            Ok(BoxShadow {
                x_offset,
                y_offset,
                blur,
                spread,
                color,
                inset,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_parse, box_shadow, color};

    assert_parse! {
        BoxShadow, length_value,

        custom {
            success {
                "10px 20px" => box_shadow!(
                    Length::px(10.0),
                    Length::px(20.0),
                    None,
                    None,
                    None,
                    false,
                ),
                "10px 20px 30px 40px red inset" => box_shadow!(
                    Length::px(10.0),
                    Length::px(20.0),
                    Some(Length::px(30.0)),
                    Some(Length::px(40.0)),
                    Some(color!(255, 0, 0)),
                    true,
                ),
            }

            failure {
                "test",
                "123",
            }
        }
    }
}
