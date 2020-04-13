use crate::ssss::tree::*;
use crate::utils::Single;

pub fn format(nodes: &[Node], config: &FormatConfig) -> String {
    let response = nodes.format(&Request {
        config,
        indentation_level: 0,
        offset: 0,
    });
    let mut text = response.text;
    text.push('\n');
    text
}

pub struct FormatConfig {
    pub limit: usize,       // Soft horizontal character limit.
    pub indentation: usize, // String that's used repeatedly to indent.
}

struct Request<'a> {
    config: &'a FormatConfig,
    indentation_level: usize,
    offset: usize,
}

impl<'a> Request<'a> {
    fn indentation_string(&self) -> String {
        " ".repeat(self.config.indentation * self.indentation_level)
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
    NotInlined(),
    Inlined(usize), // Contains the new offset.
}

trait Formattable {
    fn format(&self, request: &Request) -> Response;
}

trait MaybeInlinable {
    fn is_inlinable(&self) -> bool;
}

impl MaybeInlinable for Element {
    fn is_inlinable(&self) -> bool {
        match self {
            Element::Text(_) => true,
            Element::Block { bodies, .. } => {
                if let Some(Some(AnyData!(Element::Text(_)))) =
                    bodies.single().map(|body| body.single())
                {
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Formattable for Element {
    fn format(&self, request: &Request) -> Response {
        match self {
            Element::Text(text) => wrap_words(text.replace("{", "{{").replace("}", "}}"), request),
            Element::Block { name, bodies } => {
                if let Some(Some(AnyData!(Element::Text(text)))) =
                    bodies.single().map(|body| body.single())
                {
                    return wrap_words(
                        if request.offset > 0 {
                            format!("{}{{ {} }}", name, text)
                        } else {
                            format!("{} {{ {} }}", name, text)
                        },
                        request,
                    );
                }

                let body_request = Request {
                    indentation_level: request.indentation_level + 1,
                    offset: 0,
                    ..*request
                };
                let body_responses = bodies.iter().map(|body| body.format(&body_request));
                let mut text = "".to_string();
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
                        && offset + 5 + body_text.len() < request.config.limit - 2
                    {
                        text.push_str(" { ");
                        text.push_str(&body_text);
                        text.push_str(" }");
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
                    inlining: Inlining::NotInlined(),
                }
            }
        }
    }
}

impl Formattable for [Node] {
    fn format(&self, request: &Request) -> Response {
        // Inlining of single-text-only bodies needs to be handled at block-
        // level, because the block also needs to be inlined. So here, we know
        // for sure we're not being displayed inline.
        assert_eq!(request.offset, 0);

        // let contains_text = self.iter().any(|element| match element {
        //     Text(_) => true,
        //     Block(_, _) => false,
        // });
        let mut text = "".to_string();

        let mut offset = 0;
        for node in self {
            let response = node.element.format(&Request { offset, ..*request });
            match response.inlining {
                Inlining::NotInlined() => {
                    if !text.is_empty() && offset > request.indentation_string().len() {
                        text.push('\n');
                        text.push_str(&request.indentation_string());
                    }
                    text.push_str(&response.text);
                    text.push('\n');
                    text.push_str(&request.indentation_string());
                }
                Inlining::Inlined(new_offset) => {
                    text.push_str(&response.text);
                    offset = new_offset;
                }
            }
        }
        Response {
            text,
            inlining: Inlining::NotInlined(),
        }
    }
}

fn wrap_words(text: String, request: &Request) -> Response {
    let mut lines: Vec<String> = vec![];
    let mut line = "".to_string();
    let mut offset = request.indentation_string().len() + request.offset;
    let mut word = "".to_string();

    fn flush_word(
        request: &Request,
        lines: &mut Vec<String>,
        line: &mut String,
        offset: &mut usize,
        word: &mut String,
    ) {
        // First push the word.
        if *offset + word.len() <= request.config.limit {
            // The word still fits in the line.
            line.push_str(&word);
            *offset += word.len();
        } else {
            // The word doesn't fit in the line anymore.
            // Remove trailing spaces.
            loop {
                match line.pop() {
                    Some(' ') => continue,
                    Some(other) => {
                        line.push(other);
                        break;
                    }
                    None => break,
                }
            }
            if *offset > 0 {
                // If this is the only word in this line though, we still need
                // to squeeze it in, because it won't fit in the next lines
                // either (we remain at the same indentation).
                lines.push(std::mem::replace(line, request.indentation_string()));
                *offset = line.len();
            }
            line.push_str(&word);
            *offset += word.len();
        }
    }

    for chr in text.chars() {
        if chr.is_whitespace() {
            if !word.is_empty() {
                flush_word(request, &mut lines, &mut line, &mut offset, &mut word);
            }
            word.clear();
            line.push(chr);
            offset += 1;
        } else {
            word.push(chr);
        }
    }

    if !word.is_empty() {
        flush_word(request, &mut lines, &mut line, &mut offset, &mut word);
    }
    if offset > 0 {
        lines.push(line);
    }

    Response {
        text: lines.join("\n"),
        inlining: Inlining::Inlined(offset),
    }
}
