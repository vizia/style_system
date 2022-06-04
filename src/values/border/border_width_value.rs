use crate::{impl_from, impl_parse, BorderWidthKeyword, Length, Parse};

/// A border width value either being a [`BorderWidthKeyword`] or a [`Length`].
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse;
    use crate::LengthValue;

    assert_parse! {
        BorderWidthValue, assert_border_width_value,

        custom {
            success {
                "thin" => BorderWidthValue(Length::px(1.0)),
                "medium" => BorderWidthValue(Length::px(3.0)),
                "thick" => BorderWidthValue(Length::px(5.0)),

                "1px" => BorderWidthValue(Length::px(1.0)),
                "2in" => BorderWidthValue(Length::Value(LengthValue::In(2.0))),
                "3cm" => BorderWidthValue(Length::Value(LengthValue::Cm(3.0))),
                "4mm" => BorderWidthValue(Length::Value(LengthValue::Mm(4.0))),
                "5q" => BorderWidthValue(Length::Value(LengthValue::Q(5.0))),
                "6pt" => BorderWidthValue(Length::Value(LengthValue::Pt(6.0))),
                "7pc" => BorderWidthValue(Length::Value(LengthValue::Pc(7.0))),
                "8em" => BorderWidthValue(Length::Value(LengthValue::Em(8.0))),
                "9ex" => BorderWidthValue(Length::Value(LengthValue::Ex(9.0))),
                "10ch" => BorderWidthValue(Length::Value(LengthValue::Ch(10.0))),
                "11rem" => BorderWidthValue(Length::Value(LengthValue::Rem(11.0))),
                "12vw" => BorderWidthValue(Length::Value(LengthValue::Vw(12.0))),
                "13vh" => BorderWidthValue(Length::Value(LengthValue::Vh(13.0))),
                "14vmin" => BorderWidthValue(Length::Value(LengthValue::Vmin(14.0))),
                "15vmax" => BorderWidthValue(Length::Value(LengthValue::Vmax(15.0))),
            }

            failure {
                "test",
                "123",
                "thinpx",
            }
        }
    }
}
