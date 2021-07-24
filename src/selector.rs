
use crate::Specificity;

use std::collections::HashSet;

use bitflags::bitflags;

bitflags! {
    pub struct PseudoClasses: u8 {
        const HOVER = 1;
        const OVER = 1 << 1;
        const ACTIVE = 1 << 2;
        const FOCUS = 1 << 3;
        const DISABLED = 1 << 4;
        const CHECKED = 1 << 5;
        const SELECTED = 1 << 6;
        const CUSTOM = 1 << 7;
    }
}

impl Default for PseudoClasses {
    fn default() -> Self {
        PseudoClasses::empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Relation {
    None,
    Ancestor,
    Parent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Selector {
    pub id: Option<String>,
    pub element: Option<String>,
    pub classes: HashSet<String>,
    pub pseudo_classes: PseudoClasses,
    pub relation: Relation,
    pub asterisk: bool,
}

impl Default for Selector {
    fn default() -> Selector {
        Selector {
            id: None,
            element: None,
            classes: HashSet::new(),
            pseudo_classes: PseudoClasses::empty(),
            relation: Relation::None,
            asterisk: false,
        }
    }
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            id: None,
            element: None,
            classes: HashSet::new(),
            pseudo_classes: PseudoClasses::empty(),
            relation: Relation::None,
            asterisk: false,
        }
    }

    pub fn element(element: &str) -> Self {
        Selector {
            id: None,
            element: Some(element.to_owned()),
            classes: HashSet::new(),
            pseudo_classes: PseudoClasses::empty(),
            relation: Relation::None,
            asterisk: false,
        }
    }

    pub fn matches(&self, entity_selector: &Selector) -> bool {
        // Universal selector always matches
        if self.asterisk {
            if !self.pseudo_classes.is_empty()
                && !self.pseudo_classes.intersects(entity_selector.pseudo_classes)
            {
                return false;
            } else {
                return true;
            }
        }

        // Check for ID match
        if self.id.is_some() && self.id != entity_selector.id {
            return false;
        }

        // Check for element name match 
        if self.element.is_some() && self.element != entity_selector.element {
            return false;
        }

        // Check for classes match
        if !self.classes.is_subset(&entity_selector.classes) {
            return false;
        }

        if !self.pseudo_classes.is_empty()
            && !self.pseudo_classes.intersects(entity_selector.pseudo_classes)
        {
            return false;
        }

        if self.asterisk != entity_selector.asterisk {
            return false;
        }

        true
    }


    pub fn specificity(&self) -> Specificity {
        Specificity([
            if self.id.is_some() { 1 } else { 0 },
            (self.classes.len() + self.pseudo_classes.bits().count_ones() as usize) as u8,
            if self.element.is_some() { 1 } else { 0 },
        ])
    }


    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_owned());
        self
    }


    pub fn class(mut self, class: &str) -> Self {
        self.classes.insert(class.to_string());
        self
    }


    pub fn replace_class(&mut self, old: &str, new: &str) -> &mut Self {
        self.classes.remove(old);
        self.classes.insert(new.to_string());

        self
    }


    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = Some(id.to_owned());
        self
    }


    pub fn set_element(&mut self, element: &str) -> &mut Self {
        self.element = Some(element.to_owned());
        self
    }
}