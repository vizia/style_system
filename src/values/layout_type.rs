use crate::{impl_parse_ident, Parse};
pub use morphorm::LayoutType;

impl_parse_ident! {
    LayoutType,

    "row" => LayoutType::Row,
    "column" => LayoutType::Column,
    "grid" => LayoutType::Grid,
}
