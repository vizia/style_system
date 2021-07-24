
use std::borrow::{Borrow, Cow};

type CowStr<'a> = Cow<'a,str>;

pub enum Token<'a> {
    
    Ident(CowStr<'a>),

    AtKeyword(CowStr<'a>),

    Hash(CowStr<'a>),

    IDHash(CowStr<'a>),

    Number {
        value: f32,
    },

    Percentage {
        value: f32,
    },

    Dimension {
        value: f32,
        unit: CowStr<'a>,
    },

    WhiteSpace(&'a str),

    Comment(&'a str),

    Colon,

    Semicolon,

    Comma,

    ParenthesisBlock,

    SquareBracketBlock,

    CurlyBracketBlock,

    CloseParenthesis,

    CloseSquareBracket,

    CloseCurlyBracket,

}