use crate::Token;


pub enum TokenError {

}

pub trait Tokenizer<'a> {
    type TokenIter: Iterator<Item = Token<'a>>;
    fn tokenize(&self, input: &'a str) -> Self::TokenIter;
}

pub struct CSSTokenIter<'a> {
    input: &'a str,

    position: usize,
}

// impl<'a> Iterator for CSSTokenIter<'a> {
//     type Item = Token<'a>;

//     fn next(&mut self) -> Option<Token<'a>> {

//     }
// }

impl<'a> CSSTokenIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, ()> {
        // Consume whitespace
        let token = match self.next_byte() {
            b' ' | b'\t' => {
                self.consume_whitespace()
            },

            // b'\r' | b'\n' | b'\x0C' => {
            //     Err(())
            // },
            _=> Token::Comma,


        };

        Ok(token)
    }

    pub fn next_byte(&self) -> u8 {
        self.input.as_bytes()[self.position]
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    pub fn slice_from(&self, start_pos: usize) -> &'a str {
        &self.input[start_pos..self.position]
    }

    pub fn advance(&mut self, offset: usize) {
        self.position += offset;
    } 

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn consume_whitespace(&mut self) -> Token<'a> {
        let start_pos = self.position();
        while !self.is_eof() {
            match self.next_byte() {
                b' ' | b'\t' => {
                    self.advance(1)
                },



                _=> {
                    break;
                }
            }
        }
        Token::WhiteSpace(self.slice_from(start_pos))
    }

    // pub fn consume_newline(&mut self) -> Token<'a> {

    // }
}

struct Parser<'a> {
    input: &'a str,
    state: ParserState,
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum ParserState {
    Start,
    Element,
    PseudoClass,
    WhiteSpace,
    Block,
}

impl<'a> Parser<'a> {

    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            state: ParserState::Start,
            start: 0,
            end: 0,
        }
    }

    fn begin_token(&mut self, index: usize) {

    }

    pub fn tokenize(&mut self) {
        for (cidx, (bidx, c)) in self.input.char_indices().enumerate() {
            self.state = self.next(c, bidx);
        }
    }

    fn next(&mut self, input: char, bidx: usize) -> ParserState {
        match (&self.state, input) {
            // (_, input) if input.is_whitespace() => {
            //     println!("Whitespace");
            //     ParserState::WhiteSpace
            //     //state.next(input)
            // }

            (ParserState::Start, input) if input.is_alphanumeric() => {
                self.start = bidx;
                ParserState::Element
            }

            (ParserState::Element, input) if input.is_alphanumeric() => {
                ParserState::Element
            }

            (ParserState::Element | ParserState::WhiteSpace, input) if input == ':' => {
                self.end = bidx;
                println!("Element: {}", self.input[self.start..self.end].to_owned());
                self.start = bidx + 1;
                ParserState::PseudoClass
            }

            (ParserState::PseudoClass, input) if input.is_alphanumeric() => {
                ParserState::PseudoClass
            }

            (ParserState::PseudoClass, input) if input.is_whitespace() => {
                self.end = bidx;
                println!("PseudoClass: {}", self.input[self.start..self.end].to_owned());
                ParserState::WhiteSpace
            }

            (ParserState::WhiteSpace, input) if input == '{' => {
                ParserState::Block
            }

            (ParserState::Block, input) if input.is_whitespace() => {
                ParserState::Block
            }

            (ParserState::Block, input) if input.is_alphanumeric() || input == '-' => {
                
            }

            // (_, input) if input.is_alphanumeric() => {
            //     //println!("Character: {}", input);
            //     ParserState::Ident
            //     //state.next(input)
            // }

            (_, _) => {
                //println!("Test: {:?} {}", state, input);
                ParserState::WhiteSpace
            }
        }
    }
}

const TEST: &str = r#"
    test:hover {
        background-color: red;
    }
"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_input() {
        let count = 0;
        let mut parser = Parser::new(TEST);
        parser.tokenize();

        
    }
}