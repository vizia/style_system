


pub mod style;
pub use style::*;

pub mod specificity;
pub use specificity::*;

pub mod selector;
pub use selector::*;

pub mod rule;
pub use rule::*;

pub mod parser;
pub use parser::*;


pub fn parse_theme<S: AsRef<str>>(style: S) {

}