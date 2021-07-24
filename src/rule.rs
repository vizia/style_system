use crate::{Selector};


#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rule<P> {
    pub selectors: Vec<Selector>,
    pub properties: Vec<P>,
}

