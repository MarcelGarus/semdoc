use crate::ssss::error::SsssError;
use crate::ssss::scan::{Token, Tokenizable};
use crate::ssss::tree::*;
use crate::utils::Positioned;

use std::iter::Peekable;
use std::vec::IntoIter;

type ScannedTokens = Vec<Positioned<Token>>;

pub struct ParseResult {
    pub body: Body,
    pub errors: Vec<SsssError>,
}

impl ParseResult {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

pub fn parse(source: &str) -> ParseResult {
    let tokens: ScannedTokens = source
        .tokens()
        .skip_while(|token| matches!(token.data, Token::Whitespace(_)))
        .collect();
    for token in &tokens {
        println!("{:?}", token);
    }

    parse_body(&mut tokens.into_iter().peekable())
}

type State = Peekable<IntoIter<Positioned<Token>>>;

fn parse_body(state: &mut State) -> ParseResult {
    let mut token_buffer: ScannedTokens = vec![];
    let mut body: Vec<Node> = vec![];
    let mut errors: Vec<SsssError> = vec![];

    fn flush_buffer(body: &mut Vec<Node>, token_buffer: &mut ScannedTokens) {
        if let Some(text) = parse_text(token_buffer) {
            body.push(text);
        }
        token_buffer.clear();
    }

    // Strip leading whitespace.
    if let Some(Anywhere!(Token::Whitespace(_))) = state.peek() {
        state.next();
    }

    loop {
        let token_option = state.next();
        match token_option {
            None => break,
            Some(token) => match token.data {
                Token::Word(_) | Token::Whitespace(_) => token_buffer.push(token),
                Token::Open => {
                    let mut position = token.position;
                    // A new block begins. The name of the block is the last word
                    // in the buffer.
                    let block_name = loop {
                        match token_buffer.pop() {
                            None => {
                                errors.push(SsssError {
                                    id: "missing-block-name".to_string(),
                                    message: "Expected a block name, but found none.".to_string(),
                                    position: position.clone(),
                                });
                                break "".to_string();
                            }
                            Some(previous_token) => {
                                position.start = previous_token.position.start;
                                match previous_token.data {
                                    Token::Word(word) => break word, // We found a name!
                                    Token::Whitespace(_) => {} // Ignore any whitespace.
                                    Token::Open => panic!("No block name found, but an opening brace instead. This should never happen, because the opening brace always starts a new recursive parse_body call."),
                                    Token::Close => panic!("No block name found, but a closing brace instead. This should never happen, because if we find a closing brace, we should have stopped parsing the body and returned."),
                                }
                            }
                        }
                    };
                    flush_buffer(&mut body, &mut token_buffer);
                    // Note: parse_body already consumes the Close token.
                    let mut bodies: Vec<Body> = vec![];
                    loop {
                        let mut result = parse_body(state);
                        bodies.push(result.body);
                        errors.append(&mut result.errors);

                        // Go to the next token that's not whitespace.
                        while matches!(state.peek(), Some(Anywhere!(Token::Whitespace(_)))) {
                            token_buffer.push(state.next().unwrap());
                        }
                        if matches!(state.peek(), Some(Anywhere!(Token::Open))) {
                            // There's another body!
                            state.next();
                            token_buffer.clear();
                            continue; // Parse it!
                        } else {
                            break;
                        }
                    }
                    body.push(Node {
                        element: Element::Block {
                            name: block_name,
                            bodies,
                        },
                        metadata: Metadata {
                            position: position.clone(), //bodies.last().unwrap().data.end, // TODO:
                        },
                    });
                }
                Token::Close => {
                    // Stip trailing whitespace.
                    if let Some(Anywhere!(Token::Whitespace(_))) = token_buffer.last() {
                        token_buffer.pop();
                    }
                    flush_buffer(&mut body, &mut token_buffer);
                    return ParseResult { body, errors };
                }
            },
        }
    }
    flush_buffer(&mut body, &mut token_buffer);
    ParseResult { body, errors }
}

fn parse_text(tokens: &mut Vec<Positioned<Token>>) -> Option<Node> {
    println!("Parsing text from tokens {:?}", tokens);
    if tokens.is_empty() || tokens.iter().all(matcher!(Anywhere!(Token::Whitespace(_)))) {
        return None;
    }
    let mut text = String::new();

    for token in tokens.iter_mut().peekable() {
        match &token.data {
            Token::Open | Token::Close => {
                panic!("parse_text called with tokens that contain open or close")
            }
            Token::Word(word) => text.push_str(&word),
            Token::Whitespace(whitespaces) => {
                let mut whitespaces = whitespaces.chars().peekable();
                let mut output = "".to_string();
                loop {
                    match (whitespaces.next(), whitespaces.peek()) {
                        // A space followed by a newline translates to an
                        // actual newline.
                        (Some(' '), Some('\n')) => {
                            whitespaces.next();
                            output.push('\n');
                            break; // Ignore whitespaces in the new line.
                        }
                        // A newline without preceding space doesn't appear in
                        // the output. If there is no other whitespace before
                        // the newline, a space is inserted. Whitespaces in the
                        // new line are ignored.
                        // For example, "a<thin space>\n  b" becomes
                        // "a<thin space>b" and "a\nb" becomes "a b".
                        (Some('\n'), _) => {
                            if output.is_empty() {
                                output.push(' ');
                            }
                            break; // Ignore whitespaces in the new line.
                        }
                        (Some(chr), _) => output.push(chr),
                        (None, _) => break,
                    }
                }
                println!("Whitespaces are {:?}", output);
                text.push_str(&output);
            }
        }
    }
    Some(Node {
        element: Element::Text(text),
        metadata: Metadata {
            position: 0..0, // TODO:
        },
    })
}
