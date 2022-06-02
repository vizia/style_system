use crate::{impl_parse_ident, Parse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Ltr, // Left to right
    Rtl, // Right to left
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Ltr
    }
}

impl_parse_ident! {
    Direction,

    "ltr" => Direction::Ltr,
    "rtl" => Direction::Rtl,
}
