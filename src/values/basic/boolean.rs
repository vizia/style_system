use crate::{macros::impl_parse, Parse};

impl_parse! {
    bool,

    tokens {
        ident {
            "on" => true,
            "off" => false,
            "true" => true,
            "false" => false,
            "yes" => true,
            "no" => false,
        }
    }
}
