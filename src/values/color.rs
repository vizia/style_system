use cssparser::*;

use crate::Parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    CurrentColor,
    RGBA(RGBA),
}

impl Color {
    pub fn transparent() -> Self {
        Self::RGBA(RGBA::transparent())
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::transparent()
    }
}

impl From<cssparser::Color> for Color {
    fn from(color: cssparser::Color) -> Self {
        match color {
            cssparser::Color::CurrentColor => Color::CurrentColor,
            cssparser::Color::RGBA(rgba) => Color::RGBA(rgba),
        }
    }
}

impl<'i> Parse<'i> for Color {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
        let location = input.current_source_location();

        input
            .try_parse(cssparser::Color::parse)
            .map(|color| Ok(color.into()))?

        // if let Ok(color) = input.try_parse(cssparser::Color::parse) {
        //     Ok(color.into())
        // } else {
        //     Err(ParseError::)
        // }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_color_long_hex() {
        let input = "#8899AA";
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        let color = Color::parse(&mut parser);
        assert_eq!(color, Ok(Color::RGBA(RGBA::new(136, 153, 170, 255))));
    }

    #[test]
    fn parse_color_short_hex() {
        let input = "#889";
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        let color = Color::parse(&mut parser);
        assert_eq!(color, Ok(Color::RGBA(RGBA::new(136, 136, 153, 255))));
    }

    #[test]
    fn parse_color_rgb_func() {
        let input = "rgb(50, 60, 70)";
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        let color = Color::parse(&mut parser);
        assert_eq!(color, Ok(Color::RGBA(RGBA::new(50, 60, 70, 255))));
    }

    #[test]
    fn parse_color_rgba_func() {
        let input = "rgba(50, 60, 70, 0.5)";
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        let color = Color::parse(&mut parser);
        assert_eq!(color, Ok(Color::RGBA(RGBA::new(50, 60, 70, 128))));
    }

    #[test]
    fn parse_color_ident() {
        let input = "red";
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        let color = Color::parse(&mut parser);
        assert_eq!(color, Ok(Color::RGBA(RGBA::new(255, 0, 0, 255))));
    }
}
