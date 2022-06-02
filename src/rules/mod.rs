
pub mod style;
pub use style::*;
pub mod property;
pub use property::*;

use cssparser::*;

use crate::{ParserOptions, CustomParseError, Property};


#[derive(Debug, PartialEq, Clone)]
pub struct CssRuleList<'i>(pub Vec<CssRule<'i>>);

#[derive(Debug, PartialEq, Clone)]
pub enum CssRule<'i> {
    Style(StyleRule<'i>),
    Property(PropertyRule<'i>),
    //Keyframes(KeyframesRule<'i>),
}

impl<'i> CssRule<'i> {
    // Parse a single rule
    pub fn parse<'t>(
        input: &mut Parser<'i, 't>,
        options: &ParserOptions<'i>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>>{

    }
}

// Parse a single rule
pub fn parse_one_rule<'i, 't, R, P, E>(
    input: &mut Parser<'i, 't>,
    parser: &mut P,
) -> Result<R, ParseError<'i, E>>
where
    P: QualifiedRuleParser<'i, QualifiedRule = R, Error = E>
        + AtRuleParser<'i, AtRule = R, Error = E>,
{
    input.parse_entirely(|input| {
        input.skip_whitespace();
        let start = input.state();
        let at_keyword = if input.next_byte() == Some(b'@') {
            match *input.next_including_whitespace_and_comments()? {
                Token::AtKeyword(ref name) => Some(name.clone()),
                _ => {
                    input.reset(&start);
                    None
                }
            }
        } else {
            None
        };

        if let Some(name) = at_keyword {
            parse_at_rule(&start, name, input, parser).map_err(|e| e.0)
        } else {
            parse_qualified_rule(input, parser)
        }
    })
}

// Parse at rule
fn parse_at_rule<'i, 't, P, E>(
    start: &ParserState,
    name: CowRcStr<'i>,
    input: &mut Parser<'i, 't>,
    parser: &mut P,
) -> Result<<P as AtRuleParser<'i>>::AtRule, (ParseError<'i, E>, &'i str)>
where
    P: AtRuleParser<'i, Error = E>,
{
    let delimiters = Delimiter::Semicolon | Delimiter::CurlyBracketBlock;
    // FIXME: https://github.com/servo/rust-cssparser/issues/254
    let callback = |input: &mut Parser<'i, '_>| parser.parse_prelude(name, input);
    let result = parse_until_before(input, delimiters, callback);
    match result {
        Ok(prelude) => {
            let result = match input.next() {
                Ok(&Token::Semicolon) | Err(_) => {
                    parser.rule_without_block(prelude, start)
                        .map_err(|()| input.new_unexpected_token_error(Token::Semicolon))
                },
                Ok(&Token::CurlyBracketBlock) => {
                    // FIXME: https://github.com/servo/rust-cssparser/issues/254
                    let callback =
                        |input: &mut Parser<'i, '_>| parser.parse_block(prelude, start, input);
                    parse_nested_block(input, callback)
                },
                Ok(_) => unreachable!(),
            };
            result.map_err(|e| (e, input.slice_from(start.position())))
        },
        Err(error) => {
            let end_position = input.position();
            match input.next() {
                Ok(&Token::CurlyBracketBlock) | Ok(&Token::Semicolon) | Err(_) => {}
                _ => unreachable!(),
            };
            Err((error, input.slice(start.position()..end_position)))
        },
    }
}

fn parse_qualified_rule<'i, 't, P, E>(
    input: &mut Parser<'i, 't>,
    parser: &mut P,
) -> Result<<P as QualifiedRuleParser<'i>>::QualifiedRule, ParseError<'i, E>>
where
    P: QualifiedRuleParser<'i, Error = E>,
{
    let start = input.state();
    // FIXME: https://github.com/servo/rust-cssparser/issues/254
    let callback = |input: &mut Parser<'i, '_>| parser.parse_prelude(input);
    let prelude = parse_until_before(input, Delimiter::CurlyBracketBlock, callback);
    match *input.next()? {
        Token::CurlyBracketBlock => {
            // Do this here so that we consume the `{` even if the prelude is `Err`.
            let prelude = prelude?;
            // FIXME: https://github.com/servo/rust-cssparser/issues/254
            let callback = |input: &mut Parser<'i, '_>| parser.parse_block(prelude, &start, input);
            parse_nested_block(input, callback)
        }
        _ => unreachable!(),
    }
}

mod tests {
    use super::*;

    const CSS_EXAMPLE: &str = r#"
        button {
            background-color: blue;
        }
    "#;

    #[test]
    fn parse_rule() {
        let input = ParserInput::new(CSS_EXAMPLE);
        let mut parser = Parser::new(&mut input);
        let rule_parser = RuleParser::new();
    }
}
