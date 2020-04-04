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

trait IsWhitespace {
    fn is_whitespace(&self) -> bool;
}

impl IsWhitespace for TokenKind {
    fn is_whitespace(&self) -> bool {
        match self {
            Whitespace | Newline => true,
            _ => false,
        }
    }
}

fn determine_token_kind(character: char) -> TokenKind {
    match character {
        '{' => Open,
        '}' => Close,
        '\n' => Newline,
        ' ' | '\t' => Whitespace,
        _ => Word,
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Token {
    kind: TokenKind,
    value: String,
    position: Range<usize>,
}

fn scan(source: &str) -> Vec<Token> {
    let chars = source.chars();
    let mut tokens: Vec<Token> = vec![];
    let mut current_kind: Option<TokenKind> = None;
    let mut buffer: Vec<char> = vec![];
    let mut start: usize = 0;

    for (end, new_char) in chars.enumerate() {
        let next_kind = determine_token_kind(new_char);

        if current_kind == Some(next_kind) {
            buffer.push(new_char);
        } else {
            if let Some(previous_kind) = current_kind {
                tokens.push(Token {
                    kind: previous_kind,
                    value: buffer.into_iter().collect::<String>(),
                    position: start..end,
                });
            }
            buffer = vec![new_char];
            current_kind = Some(next_kind);
            start = end;
        }
    }

    if let Some(last_kind) = current_kind {
        tokens.push(Token {
            kind: last_kind,
            value: buffer.into_iter().collect::<String>(),
            position: start..source.len(),
        });
    }
    tokens
}

#[derive(Debug, Eq, PartialEq)]
pub enum Element {
    Text(String),
    Block(String, Content),
}
type Content = Vec<Element>;
use Element::*;

pub fn parse(source: &str) -> Content {
    let tokens = scan(source);
    let tokens: Vec<Token> = tokens
        .into_iter()
        .skip_while(|token| token.kind.is_whitespace())
        .collect();

    parse_content(&mut tokens.into_iter().peekable())
}

fn parse_content(state: &mut Peekable<IntoIter<Token>>) -> Content {
    let mut token_buffer: Vec<Token> = vec![];
    let mut content: Content = vec![];

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
                        content.push(text);
                    }
                    token_buffer = vec![];
                    let block_children = parse_content(state);
                    content.push(Block(block_name, block_children));
                }
                Close => {
                    if let Some(text) = parse_text(token_buffer) {
                        content.push(text);
                    }
                    return content;
                }
            },
            None => {
                if let Some(text) = parse_text(token_buffer) {
                    content.push(text);
                }
                return content;
            }
        }
    }
}

fn parse_text(tokens: Vec<Token>) -> Option<Element> {
    if tokens.is_empty() || tokens.iter().all(|token| token.kind.is_whitespace()) {
        return None;
    }
    let mut text_parts: Vec<String> = vec![];
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        match token.kind {
            Open | Close => panic!("parse_text called with tokens that contain open or close"),
            Word => text_parts.push(token.value),
            Whitespace => {
                if let Some(next) = tokens.peek() {
                    if next.kind == Newline {
                        text_parts.push("\n".to_string());
                        continue;
                    }
                }
                text_parts.push(token.value)
            }
            Newline => {
                if let Some(next) = tokens.peek() {
                    if next.kind == Whitespace {
                        tokens.next();
                    }
                }
            }
        }
    }
    Some(Text(text_parts.join("").trim().to_string()))
}
