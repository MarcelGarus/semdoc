use crate::utils::Positioned;
use std::iter::Peekable;

pub trait Tokenizable {
    /// Returns a higher-level representation of this source. Each [Token]
    /// represents a part of this source.
    fn tokens(&self) -> TokenIter;
}

/// A higher-level abstraction of the source.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    Open,               // An opening brace "{".
    Close,              // A closing brace "}".
    Whitespace(String), // Whitespace characters.
    Word(String),       // Actual content, like "section" or a "{" encoded as "{{".
}

impl Tokenizable for str {
    fn tokens(&self) -> TokenIter {
        TokenIter::from_source(self)
    }
}

pub struct TokenIter<'a> {
    source: Peekable<std::str::Chars<'a>>,
    offset: usize,
}

impl TokenIter<'_> {
    fn from_source(source: &str) -> TokenIter {
        TokenIter {
            source: source.chars().peekable(),
            offset: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.offset += 1;
        self.source.next()
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Positioned<Token>;

    fn next(&mut self) -> Option<Positioned<Token>> {
        let start = self.offset;
        let current = self.advance()?;
        let token = match (current, self.source.peek()) {
            ('{', Some('{')) => {
                self.advance();
                Token::Word("{".to_string())
            }
            ('}', Some('}')) => {
                self.advance();
                Token::Word("}".to_string())
            }
            ('{', _) => Token::Open,
            ('}', _) => Token::Close,
            (chr, _) if chr.is_whitespace() => {
                let mut whitespace = chr.to_string();
                loop {
                    match self.source.peek() {
                        Some(chr) if chr.is_whitespace() => {
                            whitespace.push(*chr);
                            self.advance();
                        }
                        _ => break Token::Whitespace(whitespace),
                    }
                }
            }
            _ => {
                fn belongs_to_word(chr: char) -> bool {
                    chr != '{' && chr != '}' && !chr.is_whitespace()
                }
                let mut word = "".to_string();
                word.push(current);
                while self
                    .source
                    .peek()
                    .map(|chr| belongs_to_word(*chr))
                    .unwrap_or(false)
                {
                    word.push(self.advance().expect("Scanning error")) // TODO
                }
                Token::Word(word)
            }
        };
        Some(Positioned {
            data: token,
            position: start..self.offset,
        })
    }
}
