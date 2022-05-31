use crate::{macros::define_enum_value, Parse};

define_enum_value! {
    /// Determines how nodes will be positioned when directed by the parent.
    pub enum LayoutType {
        /// Stack child elements horizontally.
        "row": Row,
        /// Stack child elements vertically.
        "column": Column,
        /// Position child elements into specified rows and columns.
        "grid": Grid,
    }
}

impl LayoutType {
    pub fn direction(&self) -> Option<Direction> {
        match self {
            LayoutType::Row => Some(Direction::X),
            LayoutType::Column => Some(Direction::Y),
            LayoutType::Grid => None,
        }
    }
}

impl Default for LayoutType {
    fn default() -> Self {
        LayoutType::Column
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    X,
    Y,
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::X => Direction::Y,
            Direction::Y => Direction::X,
        }
    }
}
