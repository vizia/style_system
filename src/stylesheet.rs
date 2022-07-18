use crate::{parser::rule::RuleParser, rule::StyleRule, CustomParseError};
use cssparser::*;

#[derive(Debug)]
pub struct StyleSheet {
    // TODO Use this
    #[allow(dead_code)]
    rules: Vec<StyleRule>,
}

impl StyleSheet {
    pub fn from_string<'i>(
        string: &'i str,
    ) -> Result<Self, (ParseError<'i, CustomParseError<'i>>, &str)> {
        let mut input = ParserInput::new(string);
        let mut parser = Parser::new(&mut input);
        let rule_parser = RuleParser::new();
        let parsed_rules = RuleListParser::new_for_stylesheet(&mut parser, rule_parser).collect();

        match parsed_rules {
            Ok(rules) => Ok(Self { rules }),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSS_EXAMPLE: &str = r#"
button label {
    left: 10%;
    right: 20px;
    top: 30in;
    bottom: 40cm;
    min-left: 50mm;
    max-left: 60q;
    background-color: white;
    width: auto;
    height: 30px;
    border-radius: 3px;
    child-space: 1st;
    child-left: 10px;
    child-right: 10px;
    border-width: 1px;
    border-color: #e5e5e5;
    outer-shadow: 0px 1px 1px #00000055;
    overflow: visible;
    position-type: parent-directed;
    left: 10%;
    position-type: self-directed;
    opacity: 10%;
    opacity: 000.10;
    background-color: red;
    layout-type: grid;
    layout-type: column;
    layout-type: row;
    font-size: 10;
    font-size: large;
    font-size: medium;
    font-size: small;
    font: "test 234234 2332 4";
    font: "";
    background-image: "23487";
    display: none;
    display: flex;
    overflow: hidden;
    visibility: invisible;
    overflow: visible;
    visibility: visible;
    text-wrap: false;
    text-wrap: yes;
    text-wrap: on;
    cursor: default;
    cursor: move;
    cursor: crosshair;
    border-top-right-shape: bevel;
    outer-shadow: 10px 8px 10px #123456;
    z-index: 9999900;
    transition: test 1s, test2 2s, test3 3s 4s;
    background-color: weriwrgba(12, 13, 14, 0.1);
    backgrond-color: hsla(120, 100%, 50%, 0.1);
    outline-color: red;
    outline-bottom-color: #00FF00;
    outline-corner-shape: round;
    outline-top-right-shape: bevel;
    outline-radius: 2px;
    outline-bottom-left-radius: 10px;
    transform: rotate(10deg);
    transform: scale(20%, 30);
    transform: scale(20%, 30) rotate(50rad);
    transform: scale(20%, 30) rotate(50rad) skew(50deg, 30turn);
    translate: 10px, 20px;
    rotate: 20deg;
    scale: 20%, 10;
    border-corner-shape: bevel round round bevel;
    border-top-left-shape: round;
    border-top-right-shape: bevel;
}

test {
    background-color: #123123;
}
"#;

    #[test]
    fn parse_stylsheet() {
        let style_sheet = StyleSheet::from_string(CSS_EXAMPLE);
        println!("{:#?}", style_sheet);
    }
}

// use cssparser::*;

// use crate::{CssRule, CssRuleList, ParserOptions};

// #[derive(Debug)]
// pub struct StyleSheet<'i> {
//     pub rules: CssRuleList<'i>,
//     pub sources: Vec<String>,
//     options: ParserOptions,
// }

// impl<'i> StyleSheet<'i> {
//     pub fn new(sources: Vec<String>, rules: CssRuleList, options: ParserOptions) -> StyleSheet {
//         StyleSheet {
//             sources,
//             rules,
//             options,
//         }
//     }

//     pub fn parse(
//         filename: String,
//         code: &'i str,
//         options: ParserOptions,
//     ) -> Result<StyleSheet<'i>, Error<ParserError<'i>>> {
//         let mut input = ParserInput::new(&code);
//         let mut parser = Parser::new(&mut input);
//         let rule_list_parser =
//             RuleListParser::new_for_stylesheet(&mut parser, TopLevelRuleParser::new(&options));

//         let mut rules = vec![];
//         for rule in rule_list_parser {
//             let rule = match rule {
//                 Ok((_, CssRule::Ignored)) => continue,
//                 Ok((_, rule)) => rule,
//                 Err((e, _)) => return Err(Error::from(e, filename)),
//             };

//             rules.push(rule)
//         }

//         Ok(StyleSheet {
//             sources: vec![filename],
//             rules: CssRuleList(rules),
//             options,
//         })
//     }
// }
