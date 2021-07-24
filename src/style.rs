use morphorm::Hierarchy;

use morphorm;


pub trait Rules<'a> {
    type Item: 'a;
    type RuleIter: Iterator<Item = &'a Self::Item>;

    fn iter(&'a self) -> Self::RuleIter; 
}

// pub fn style<'a, H, R>(hierarchy: &'a H, rules: Vec<Rule>) 
// where
//     H: Hierarchy<'a>,
//     R: Rules<'a>,

// {
//     // for node in hierarchy.down_iter() {

//     //     'rule_loop: for rule in rules.iter() {

//     //         'selector_loop: for selector in rule.selectors
//     //     }
//     // }
// }