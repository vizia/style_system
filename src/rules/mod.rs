
pub mod style;
pub use style::*;

#[derive(Debug, PartialEq, Clone)]
pub enum CssRule<'i> {
    Style(StyleRule<'i>),
    //Keyframes(KeyframesRule<'i>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CssRuleList<'i>(pub Vec<CssRule<'i>>);
