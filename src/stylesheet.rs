

use cssparser::*;

use crate::{CssRuleList, CssRule, ParserOptions};

#[derive(Debug)]
pub struct StyleSheet<'i> {
    pub rules: CssRuleList<'i>,
    pub sources: Vec<String>,
    options: ParserOptions,
}

impl<'i> StyleSheet<'i> {

    pub fn new(sources: Vec<String>, rules: CssRuleList, options: ParserOptions) -> StyleSheet {
        StyleSheet {
          sources,
          rules,
          options
        }
      }

    pub fn parse(filename: String, code: &'i str, options: ParserOptions) -> Result<StyleSheet<'i>, Error<ParserError<'i>>> {
        let mut input = ParserInput::new(&code);
        let mut parser = Parser::new(&mut input);
        let rule_list_parser = RuleListParser::new_for_stylesheet(&mut parser, TopLevelRuleParser::new(&options));
    
        let mut rules = vec![];
        for rule in rule_list_parser {
            let rule = match rule {
                Ok((_, CssRule::Ignored)) => continue,
                Ok((_, rule)) => rule,
                Err((e, _)) => return Err(Error::from(e, filename))
            };
    
          rules.push(rule)
        }
    
        Ok(StyleSheet {
          sources: vec![filename],
          rules: CssRuleList(rules),
          options
        })
      }
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stylsheet() {
        
    }
}