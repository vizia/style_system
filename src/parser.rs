use std::{fmt::Result, string::ParseError, borrow::Cow};

use cssparser::*;

use crate::{CustomIdent, CustomParseError};

#[derive(Default, Clone, Debug)]
pub struct ParserOptions {
  pub nesting: bool,
  pub custom_media: bool,
  pub css_modules: bool,
  pub source_index: u32
}

// pub struct RuleParser<'a, 'i> {

// }

// impl<'a, 'i> RuleParser<'a, 'i> {
//     pub fn new() -> Self {
//         Self {

//         }
//     }
// }


#[cfg(tests)]
mod tests {
    #[test]
    fn parse_at_rule_keyframes() {
        let TEST: &str = "@keyframes";
    }
}

// pub struct StyleRuleParser<'a, 'i> {
//     default_namespace: &'a Option<CowRcStr<'a>>,

// }

// impl<'a,'i> DeclarationParser<'i> for StyleRuleParser<'a,'i> {
//     type Declaration = ();
//     type Error = ();

//     fn parse_valye<'t>(
//         &mut self,
//         name: CowRcStr<'i>,
//         input: &mut Parser<'i,t>,
//     ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {

//     }

// }