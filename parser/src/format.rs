use crate::parse::{Element, Element::*};
use crate::utils::Single;

pub fn format(elements: &[Element], context: &Context) -> String {
    let response = elements.format(&Request {
        context,
        indentation_level: 0,
        offset: 0,
    });
    response.text
}

pub struct Context {
    pub limit: usize,        // Soft horizontal character limit.
    pub indentation: String, // String that's used repeatedly to indent.
}

struct Request<'a> {
    context: &'a Context,
    indentation_level: usize,
    offset: usize,
}

impl<'a> Request<'a> {
    fn indentation_string(&self) -> String {
        self.context.indentation.repeat(self.indentation_level)
    }
}

struct Response {
    text: String,
    inlining: Inlining,
}

// This is a trivial block with only one body that contains text. These
// kind of blocks get inlined:
// ```mlml
// This is text with some bold{blocks that are
// formatted inline}. Notice that italic{their
// body text may break}.
// ```
enum Inlining {
    NotInlinable(),
    Inlined(usize), // Contains the new offset.
}

trait Formattable {
    fn format(&self, request: &Request) -> Response;
}

impl Formattable for Element {
    fn format(&self, request: &Request) -> Response {
        match self {
            Text(text) => {
                let words: Vec<&str> = text.split_whitespace().collect();
                wrap_parts(&words, request)
            }
            Block(name, bodies) => {
                if let Some(Some(Text(text))) = bodies.single().map(|body| body.single()) {
                    let text = format!("{}{{ {} }}", name, text);
                    let parts: Vec<&str> = text.split_whitespace().collect();
                    wrap_parts(&parts, request)
                } else {
                    let body_request = Request {
                        indentation_level: request.indentation_level + 1,
                        offset: 0,
                        ..*request
                    };
                    let body_responses = bodies.iter().map(|body| body.format(&body_request));
                    let mut text = request.indentation_string();
                    text.push_str(name);
                    let mut offset = text.len();
                    for response in body_responses {
                        let body_text = response.text;
                        // Check if the body would fit into the same line. That
                        // means, it is only one line and if we would surround
                        // it with " { " and " }" (5 characters in total), it
                        // would still be less than the limit and leave two
                        // characters left for the next body's start.
                        if !body_text.contains('\n')
                            && offset + 5 + body_text.len() < request.context.limit - 2
                        {
                            text.push_str(" {{");
                            text.push_str(&body_text);
                            text.push_str(" }}");
                            offset += 5 + body_text.len();
                        } else {
                            text.push_str(" {\n");
                            text.push_str(&body_request.indentation_string());
                            text.push_str(&body_text);
                            text.push_str("\n");
                            text.push_str(&request.indentation_string());
                            text.push_str("}");
                            offset = request.indentation_string().len() + 1;
                        }
                    }
                    Response {
                        text,
                        inlining: Inlining::NotInlinable(),
                    }
                }
            }
        }
    }
}

impl Formattable for [Element] {
    fn format(&self, request: &Request) -> Response {
        // Inlining of single-text-only bodies needs to be handled at block-
        // level, because the block also needs to be inlined. So here, we know
        // for sure we're not being displayed inline.
        assert_eq!(request.offset, 0);

        let mut text = "".to_string();
        let mut offset = 0;

        for element in self {
            let response = element.format(&Request { offset, ..*request });
            match response.inlining {
                Inlining::NotInlinable() => {
                    text.push_str("\n");
                    text.push_str(&request.indentation_string());
                    text.push_str(&response.text);
                }
                Inlining::Inlined(new_offset) => {
                    text.push_str(&response.text);
                    offset = new_offset;
                }
            }
        }

        Response {
            text,
            inlining: Inlining::NotInlinable(),
        }
    }
}

fn wrap_parts(parts: &[&str], request: &Request) -> Response {
    let indentation = request.indentation_string();
    let mut lines: Vec<String> = vec![];
    let mut line = "".to_string();
    let mut offset = request.offset;
    for part in parts {
        if offset + 1 + part.len() <= request.context.limit {
            // The word still fits in the line.
            line.push(' ');
            line.push_str(&part);
            offset += 1 + part.len();
        } else {
            // The word doesn't fit in the line anymore.
            if offset != 0 {
                // If this is the only word in this line though, we still need
                // to squeeze it in, because it won't fit in the next lines
                // either (we remain at the same indentation).
                lines.push(line);
                line = indentation.clone();
                offset = indentation.len();
            }
            line.push_str(&part);
            offset += part.len();
        }
    }

    Response {
        text: lines.join("\n"),
        inlining: Inlining::Inlined(offset),
    }
}
