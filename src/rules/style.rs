
use selectors::SelectorList;

pub struct StyleRule<'i> {
    pub selectors: SelectorList<'i, Selectors>,
    pub declarations: DeclarationBlock<'i>,
    pub rules: CssRuleList<'i>,
    pub loc: Location,
}