use crate::{parser::declaration::DeclarationParser, CustomParseError, Property};
use cssparser::{
    AtRuleParser, DeclarationListParser, ParseError, Parser, ParserState, QualifiedRuleParser,
    Token,
};

#[derive(Debug)]
pub struct RuleParser;

impl RuleParser {
    pub fn new() -> Self {
        RuleParser
    }
}

impl<'i> QualifiedRuleParser<'i> for RuleParser {
    type Prelude = Vec<Selector>;
    type QualifiedRule = StyleRule;
    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let mut selectors: Vec<Selector> = Vec::new();

        while let Ok(token) = input.next() {
            match token {
                Token::Ident(ident) => {
                    selectors.push(Selector::new(ident.to_string()));
                }
                _ => {}
            }
        }

        Ok(selectors)
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let decl_parser = DeclarationParser;
        let parsed_properties = DeclarationListParser::new(input, decl_parser);
        let mut properties: Vec<Property> = Vec::new();
        let mut errors: Vec<(ParseError<CustomParseError>, &str)> = Vec::new();

        for property in parsed_properties {
            match property {
                Ok(prop) => properties.push(prop),
                Err(error) => errors.push(error),
            }
        }

        if errors.len() > 0 {
            for error in errors {
                eprintln!(
                    "ERROR: Error while parsing `{}` at {}:{}:\n{:?}\n",
                    error.1, error.0.location.line, error.0.location.column, error.0.kind
                );
            }
        }

        Ok(StyleRule {
            id: Rule(0),
            selectors: prelude,
            properties,
        })
    }
}

impl<'i> AtRuleParser<'i> for RuleParser {
    type Prelude = ();
    type AtRule = StyleRule;
    type Error = CustomParseError<'i>;
}

#[derive(Debug)]
pub struct StyleRule {
    pub id: Rule,
    pub selectors: Vec<Selector>,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub struct Selector {
    pub name: String,
}

impl Selector {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct Rule(u32);
