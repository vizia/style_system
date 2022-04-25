use cssparser::*;
use parcel_selectors::{SelectorImpl, parser::Selector};

use crate::{PseudoClass, PseudoElement, CustomParseError, pseudoclass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selectors;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SelectorString<'a>(pub CowRcStr<'a>);


impl<'a> std::convert::From<CowRcStr<'a>> for SelectorString<'a> {
    fn from(s: CowRcStr<'a>) -> SelectorString<'a> {
        SelectorString(s.into())
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct SelectorIdent<'i>(pub CowRcStr<'i>);

impl<'a> std::convert::From<CowRcStr<'a>> for SelectorIdent<'a> {
    fn from(s: CowRcStr<'a>) -> SelectorIdent {
        SelectorIdent(s.into())
    }
}


impl<'i> SelectorImpl<'i> for Selectors {
    type AttrValue = SelectorString<'i>;
    type Identifier = SelectorIdent<'i>;
    type LocalName = SelectorIdent<'i>;
    type NamespacePrefix = SelectorIdent<'i>;
    type NamespaceUrl = SelectorIdent<'i>;
    type BorrowedNamespaceUrl = SelectorIdent<'i>;
    type BorrowedLocalName = SelectorIdent<'i>;
  
    type NonTSPseudoClass = PseudoClass<'i>;
    type PseudoElement = PseudoElement<'i>;
  
    type ExtraMatchingData = ();
}

pub struct SelectorParser;

impl<'i> parcel_selectors::parser::Parser<'i> for SelectorParser {
    type Impl = Selectors;
    type Error = CustomParseError<'i>;

    fn parse_non_ts_pseudo_class(
        &self,
        _: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<PseudoClass<'i>, ParseError<'i, Self::Error>> {
        use PseudoClass::*;
        let pseudo_class = match_ignore_ascii_case! { &name,
            "hover" => Hover,
            "active" => Active,
            "focus" => Focus,
            "enabled" => Enabled,
            "disabled" => Disabled,
            "read-only" => ReadOnly,
            "read-write" => ReadWrite,
            "default" => Default,
            "checked" => Checked,
            "indeterminate" => Indeterminate,
            "blank" => Blank,
            "valid" => Valid,
            "invalid" => Invalid,
            "in-range" => InRange,
            "out-of-range" => OutOfRange,
            "required" => Required,
            "optional" => Optional,
            "user-valid" => UserValid,
            "user-invalid" => UserInvalid,

            _ => Custom(name.into())
        
        };

        Ok(pseudo_class)
    }
}


#[cfg(test)]
mod tests {
    use parcel_selectors::SelectorList;

    use super::*;

    const VALID_ELEMENT_SELECTOR: &str = "button";

    //pub struct CompiledSelector(Selector<Selectors>);

    #[test]
    fn parse_element_selector() {
        let mut parser_input = ParserInput::new(&VALID_ELEMENT_SELECTOR);
        let mut parser = Parser::new(&mut parser_input);
        let result = SelectorList::parse(&SelectorParser, &mut parser, parcel_selectors::parser::NestingRequirement::None);
        println!("{:?}", result.is_ok());
        //assert_eq!(3, result);
    }
}