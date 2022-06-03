use crate::{impl_parse, Parse};
pub use morphorm::PositionType;

impl_parse! {
    PositionType,

    tokens {
        ident {
            "self-directed" => PositionType::SelfDirected,
            "parent-directed" => PositionType::ParentDirected,
        }
    }
}
