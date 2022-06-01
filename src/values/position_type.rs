use crate::{impl_parse_ident, Parse};
pub use morphorm::PositionType;

impl_parse_ident! {
    PositionType,
    "self-directed" => PositionType::SelfDirected,
    "parent-directed" => PositionType::ParentDirected,
}
