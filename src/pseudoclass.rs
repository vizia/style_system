
use cssparser::*;

use crate::Selectors;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClass<'i> {
    Hover,
    Active,
    Focus,

    Enabled,
    Disabled,
    ReadOnly,
    ReadWrite,
    PlaceHolderShown,
    Default,
    Checked,
    Indeterminate,
    Blank,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    Required,
    Optional,
    UserValid,
    UserInvalid,

    Custom(CowRcStr<'i>),
}

impl<'i> parcel_selectors::parser::NonTSPseudoClass<'i> for PseudoClass<'i> {
    type Impl = Selectors;
  
    fn is_active_or_hover(&self) -> bool {
        matches!(*self, PseudoClass::Active | PseudoClass::Hover)
    }
  
    fn is_user_action_state(&self) -> bool {
        matches!(*self, PseudoClass::Active | PseudoClass::Hover | PseudoClass::Focus)
    }
}