use crate::{impl_parse, Parse};
pub use morphorm::LayoutType;

impl_parse! {
    LayoutType,

    tokens {
        ident {
            "row" => LayoutType::Row,
            "column" => LayoutType::Column,
            "grid" => LayoutType::Grid,
        }
    }
}
