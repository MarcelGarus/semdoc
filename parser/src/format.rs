use crate::parse::{Element, Element::*};
use crate::utils::Single;

pub fn format(elements: &[Element]) -> String {
    let mut context = FormatContext {
        // limit: 80,
        string: "".to_string(),
        indention: 0,
        offset: 0,
    };
    elements.format(&mut context);
    context.string
}

type Indention = usize;

struct FormatContext {
    // limit: usize, // Soft horizontal character limit.
    string: String,
    indention: Indention,
    offset: usize, // Offset from the left.
}

impl FormatContext {
    fn push(&mut self, string: &str) {
        self.ensure_indented();
        self.string.push_str(string);
    }
    fn ensure_indented(&mut self) {
        if self.offset == 0 {
            self.push(&"  ".repeat(self.indention))
        }
    }
}

trait Formattable {
    fn format(&self, context: &mut FormatContext);
}

impl Formattable for &[Element] {
    fn format(&self, context: &mut FormatContext) {
        for (i, element) in self.iter().enumerate() {
            if i > 0 {
                context.push(" ");
            }
            element.format(context);
        }
    }
}

impl Formattable for Element {
    fn format(&self, context: &mut FormatContext) {
        match self {
            Text(text) => format_text(text, context),
            Block(name, content) => format_block(name, content, context),
        }
    }
}

fn format_text(text: &str, context: &mut FormatContext) {
    context.push(&text)
}

fn format_block(name: &str, content: &[Element], context: &mut FormatContext) {
    if content.is_empty() {
        context.push(&format!("{}{{}}", name));
    } else if let Some(single_element) = content.single() {
        context.push(&format!("{}{{{}}}", name, single_element.format(context)));
        format!(
            "{}{}{{{}}}",
            context.indent(),
            name,
            format_text(&text, context)
        )
    } else {
        let indent = context.indent();
        context.indention += 1;
        format!("{}{} {{\n{}\n}}", indent, name, &content.format(context))
    }
}
