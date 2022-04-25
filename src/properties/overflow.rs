use cssparser::*;
use crate::{enum_property, Parse, CustomParseError};

enum_property! {
    pub enum OverflowKeyword {
        Visible,
        Hidden,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Overflow {
    pub x: OverflowKeyword,
    pub y: OverflowKeyword,
}

impl<'i> Parse<'i> for Overflow {
    fn parse<'t>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let x = OverflowKeyword::parse(input)?;
        let y = input.try_parse(OverflowKeyword::parse).unwrap_or_else(|_| x.clone());
        Ok(Overflow { x, y })
    }
}

enum_property! {
    pub enum TextOverflow {
        Clip,
        Ellipsis,
    }
}


