use crate::{macros::impl_parse_try_parse, Length, Parse, Rect};

#[derive(Debug, Clone, PartialEq)]
pub struct BorderRadius {
    pub top_left_radius: Length,
    pub top_right_radius: Length,
    pub bottom_right_radius: Length,
    pub bottom_left_radius: Length,
}

impl_parse_try_parse!(BorderRadius => Rect<Length>);

impl From<Rect<Length>> for BorderRadius {
    fn from(rect: Rect<Length>) -> Self {
        BorderRadius {
            top_left_radius: rect.0,
            top_right_radius: rect.1,
            bottom_right_radius: rect.2,
            bottom_left_radius: rect.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::assert_parse_value;

    macro_rules! border_radius {
        ($top_left: expr, $top_right: expr, $bottom_right: expr, $bottom_left: expr$(,)?) => {{
            use $crate::Length::*;
            use $crate::LengthValue::*;

            BorderRadius {
                top_left_radius: Value($top_left),
                top_right_radius: Value($top_right),
                bottom_right_radius: Value($bottom_right),
                bottom_left_radius: Value($bottom_left),
            }
        }};
    }

    #[test]
    fn test_sucess() {
        assert_parse_value!(
            BorderRadius,
            "10px",
            border_radius!(Px(10.0), Px(10.0), Px(10.0), Px(10.0))
        );

        assert_parse_value!(
            BorderRadius,
            "10px 20px",
            border_radius!(Px(10.0), Px(20.0), Px(10.0), Px(20.0))
        );

        assert_parse_value!(
            BorderRadius,
            "10px 20px 30px",
            border_radius!(Px(10.0), Px(20.0), Px(30.0), Px(20.0))
        );

        assert_parse_value!(
            BorderRadius,
            "10px 20px 30px 40px",
            border_radius!(Px(10.0), Px(20.0), Px(30.0), Px(40.0))
        );
    }

    #[test]
    fn test_failure() {
        assert_parse_value!(BorderRadius, "px");
        assert_parse_value!(BorderRadius, "10px 20px 30px 40px 50px");
    }
}
