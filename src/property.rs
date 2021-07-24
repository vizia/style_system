use morphorm::{LayoutType, PositionType, Units};


#[derive(Clone, Debug, PartialEq)]
pub enum Property {
    LayoutType(LayoutType),
    PositionType(PositionType),

    
}