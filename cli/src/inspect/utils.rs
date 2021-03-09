use colored::Colorize;

pub fn format_children_strings(children: &[String]) -> String {
    children
        .iter()
        .enumerate()
        .map(|(index, child)| {
            let (first_line_prefix, rest_prefix) = match index == children.len() - 1 {
                false => ("├─", "│ "),
                true => ("└─", "  "),
            };
            let content = textwrap::indent(&child, rest_prefix);
            format!("{}{}", first_line_prefix, &content[rest_prefix.len()..])
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn kind_to_name(kind: u64) -> String {
    match kind {
        0 => "Empty",
        1 => "Text",
        2 => "Section",
        3 => "DenseSequence",
        4 => "SplitSequence",
        _ => "Unknown",
    }
    .to_owned()
}

pub fn singular_or_plural(
    amount: usize,
    singular: &'static str,
    plural: &'static str,
) -> &'static str {
    match amount {
        1 => singular,
        _ => plural,
    }
}

pub trait AsciiOrDot {
    /// Returns Some(char) if `this` is a visible ASCII character and None otherwise.
    fn ascii_or_none(&self) -> Option<char>;
}
impl AsciiOrDot for u8 {
    fn ascii_or_none(&self) -> Option<char> {
        if (32..=126).contains(self) {
            Some(*self as char)
        } else {
            None
        }
    }
}

pub trait RoundUpToMultipleOf {
    fn round_up_to_multiple_of(&self, number: Self) -> Self;
}
impl RoundUpToMultipleOf for usize {
    fn round_up_to_multiple_of(&self, number: Self) -> Self {
        self + (number - self % number) % number
    }
}

/// Formats payload bytes in both hex and ascii.
pub fn format_payload_bytes(bytes: &[u8], width: usize) -> String {
    let hex = bytes
        .iter()
        .map(|byte| format!("{:02x}", byte).bright_cyan().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let ascii = bytes
        .iter()
        .map(|byte| {
            byte.ascii_or_none()
                .map(|it| it.to_string().blue())
                .unwrap_or_else(|| '.'.to_string().red())
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("");
    textwrap::wrap(&format!("{}\n{}", hex, ascii), width)
        .iter()
        .map(|line| format!("{}", line))
        .collect::<Vec<_>>()
        .join("\n")
}
