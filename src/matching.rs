

mod test {
    use std::{collections::HashMap, hash::Hash};

    use cssparser::*;
    use parcel_selectors::{matching::{matches_selector, matches_selector_list}, OpaqueElement, context::{MatchingMode, MatchingContext, QuirksMode}, SelectorList};

    use crate::{Selectors, SelectorIdent, SelectorParser, CustomParseError};

    fn parse<'i>(
        input: &'i str,
    ) -> Result<SelectorList<Selectors>, ParseError<'i, CustomParseError<'i>>> {
        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);
        SelectorList::parse(
            &SelectorParser {
                default_namespace: &None,
                is_nesting_allowed: true,
            },
            &mut parser,
            parcel_selectors::parser::NestingRequirement::None,
        )
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Entity(u32);

    #[derive(Debug)]
    pub struct Store {
        element: HashMap<Entity, String>,
    }

    #[derive(Debug, Clone)]
    pub struct Node<'s> {
        entity: Entity,
        store: &'s Store,
    }

    impl<'i, 's> parcel_selectors::Element<'i> for Node<'s> {
        type Impl = Selectors;

        fn opaque(&self) -> parcel_selectors::OpaqueElement {
            OpaqueElement::new(self)
        }

        fn is_html_slot_element(&self) -> bool {
            false
        }

        fn parent_node_is_shadow_root(&self) -> bool {
            false
        }

        fn containing_shadow_host(&self) -> Option<Self> {
            None
        }

        fn parent_element(&self) -> Option<Self> {
            None
        }

        fn prev_sibling_element(&self) -> Option<Self> {
            None
        }

        fn next_sibling_element(&self) -> Option<Self> {
            None
        }

        fn is_empty(&self) -> bool {
            true
        }

        fn is_root(&self) -> bool {
            false
        }

        fn is_html_element_in_html_document(&self) -> bool {
            false
        }

        fn has_local_name(&self, local_name: &SelectorIdent<'i>) -> bool {
            if let Some(element) = self.store.element.get(&self.entity) {
                return element == local_name.0.as_ref();
            }

            false
        }

        fn has_namespace(&self, ns: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::BorrowedNamespaceUrl) -> bool {
            false
        }

        fn is_part(&self, name: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier) -> bool {
            false
        }

        fn imported_part(&self, name: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier) -> Option<<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier> {
            None
        }

        fn is_pseudo_element(&self) -> bool {
            false
        }

        fn is_same_type(&self, other: &Self) -> bool {
            self.store.element.get(&self.entity) == other.store.element.get(&self.entity)
        }

        fn is_link(&self) -> bool {
            false
        }

        fn has_id(&self, id: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier, case_sensitivity: parcel_selectors::attr::CaseSensitivity) -> bool {
            false
        }

        fn has_class(&self, name: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::Identifier, case_sensitivity: parcel_selectors::attr::CaseSensitivity) -> bool {
            false
        }

        fn attr_matches(&self, ns: &parcel_selectors::attr::NamespaceConstraint<&<Self::Impl as parcel_selectors::SelectorImpl<'i>>::NamespaceUrl>, local_name: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::LocalName, operation: &parcel_selectors::attr::AttrSelectorOperation<&<Self::Impl as parcel_selectors::SelectorImpl<'i>>::AttrValue>) -> bool {
            false
        }

        fn match_pseudo_element(&self, pe: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::PseudoElement, context: &mut parcel_selectors::context::MatchingContext<'_, 'i, Self::Impl>) -> bool {
            false
        }

        fn match_non_ts_pseudo_class<F>(&self, pc: &<Self::Impl as parcel_selectors::SelectorImpl<'i>>::NonTSPseudoClass, context: &mut parcel_selectors::context::MatchingContext<'_, 'i, Self::Impl>, flags_setter: &mut F) -> bool
        where
                F: FnMut(&Self, parcel_selectors::matching::ElementSelectorFlags) {
            false
        }


    }

    #[test]
    fn asterisk_match() {

        let mut store = Store {
            element: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.element.insert(root, String::from("window"));
        store.element.insert(child, String::from("button"));

        let root_node = Node {
            entity: root,
            store: &store,
        };

        let child_node = Node {
            entity: child,
            store: &store,
        };

        if let Ok(selector_list) = parse("*") {
            
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                None,
                QuirksMode::NoQuirks,
            );
            
            let result = matches_selector_list(
                &selector_list,
                &root_node,
                &mut context,
            );

            println!("Result: {}", result);
        }
    }

    #[test]
    fn element_match() {

        let mut store = Store {
            element: HashMap::new(),
        };

        let root = Entity(0);
        let child = Entity(1);

        store.element.insert(root, String::from("window"));
        store.element.insert(child, String::from("button"));

        let root_node = Node {
            entity: root,
            store: &store,
        };

        let child_node = Node {
            entity: child,
            store: &store,
        };

        if let Ok(selector_list) = parse("window") {
            
            let mut context = MatchingContext::new(
                MatchingMode::Normal,
                None,
                None,
                QuirksMode::NoQuirks,
            );
            
            let result = matches_selector_list(
                &selector_list,
                &root_node,
                &mut context,
            );

            println!("Result: {}", result);

            let result = matches_selector_list(
                &selector_list,
                &child_node,
                &mut context,
            );

            println!("Result: {}", result);
        }
    }

}