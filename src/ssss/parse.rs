use core::ops::Range;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum TokenKind {
    Open,  // {
    Close, // }
    Word,
    Whitespace,
    Newline,
}
use TokenKind::*;

impl TokenKind {
    fn is_whitespace(self) -> bool {
        match self {
            Whitespace | Newline => true,
            _ => false,
        }
    }
}

/// Every character of the input file can be directly mapped to a [TokenKind].
fn determine_token_kind(character: char) -> TokenKind {
    match character {
        '{' => Open,
        '}' => Close,
        '\n' => Newline,
        ' ' | '\t' => Whitespace,
        _ => Word,
    }
}

/// A higher-level abstraction on the source file.
#[derive(Debug, Eq, PartialEq)]
struct Token {
    kind: TokenKind,
    value: String,
    position: Range<usize>, // Position in the source file in bytes.
}

/// Truns a source into a [Vec] of [Token]s by merging adjacent characters that
/// map to the same [TokenKind] into one [Token].
fn scan(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_kind: Option<TokenKind> = None;
    let mut buffer = String::new();
    let mut start: usize = 0;

    for (end, next_char) in source.chars().enumerate() {
        let next_kind = determine_token_kind(next_char);

        if current_kind == Some(next_kind) {
            buffer.push(next_char);
        } else {
            if let Some(previous_kind) = current_kind {
                tokens.push(Token {
                    kind: previous_kind,
                    value: buffer,
                    position: start..end,
                });
            }
            buffer = format!("{}", next_char);
            current_kind = Some(next_kind);
            start = end;
        }
    }

    if let Some(last_kind) = current_kind {
        tokens.push(Token {
            kind: last_kind,
            value: buffer,
            position: start..source.len(),
        });
    }
    tokens
}

/// An [Element] in the abstract syntax tree.
#[derive(Debug, Eq, PartialEq)]
pub enum Element {
    Text(String),
    Block(String, Vec<Body>), // A block has a name and bodies.
}
pub type Body = Vec<Element>;
use Element::*;

pub fn parse(source: &str) -> Body {
    let tokens = scan(source);
    let tokens: Vec<Token> = tokens
        .into_iter()
        .skip_while(|token| token.kind.is_whitespace())
        .collect();

    parse_body(&mut tokens.into_iter().peekable())
}

fn parse_body(state: &mut Peekable<IntoIter<Token>>) -> Body {
    let mut token_buffer: Vec<Token> = vec![];
    let mut body: Body = vec![];

    loop {
        match state.next() {
            Some(token) => match token.kind {
                Word | Whitespace | Newline => token_buffer.push(token),
                Open => {
                    let block_name = loop {
                        let token = token_buffer.remove(token_buffer.len() - 1);
                        match token.kind {
                            Word => break token.value,
                            Whitespace | Newline => {}
                            Open | Close => {
                                // TODO: Block name expected.
                                println!("Block name expected.");
                            }
                        }
                    };

                    if let Some(text) = parse_text(token_buffer) {
                        body.push(text);
                    }
                    token_buffer = vec![];
                    // Note: parse_body already consumes the Close token.
                    let mut bodies: Vec<Body> = vec![parse_body(state)];
                    loop {
                        // Go to the next token that's not whitespace.
                        while state
                            .peek()
                            .map(|token| token.kind.is_whitespace())
                            .unwrap_or(false)
                        {
                            state.next();
                        }
                        if state
                            .peek()
                            .map(|token| token.kind == Open)
                            .unwrap_or(false)
                        {
                            // There's another body!
                            state.next();
                            bodies.push(parse_body(state));
                            continue;
                        } else {
                            break;
                        }
                    }
                    body.push(Block(block_name, bodies));
                }
                Close => {
                    if let Some(text) = parse_text(token_buffer) {
                        body.push(text);
                    }
                    return body;
                }
            },
            None => {
                if let Some(text) = parse_text(token_buffer) {
                    body.push(text);
                }
                return body;
            }
        }
    }
}

fn parse_text(tokens: Vec<Token>) -> Option<Element> {
    if tokens.is_empty() || tokens.iter().all(|token| token.kind.is_whitespace()) {
        return None;
    }
    let mut text = String::new();
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token.kind {
            Open | Close => panic!("parse_text called with tokens that contain open or close"),
            Word => text.push_str(&token.value),
            Whitespace => {
                if let Some(next) = tokens.peek() {
                    if next.kind == Newline {
                        text.push('\n');
                        continue;
                    }
                }
                text.push_str(&token.value)
            }
            Newline => {
                if let Some(next) = tokens.peek() {
                    if next.kind == Whitespace {
                        tokens.next();
                        text.push(' ');
                    }
                }
            }
        }
    }
    Some(Text(text.trim().to_string()))
}
