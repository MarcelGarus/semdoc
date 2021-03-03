use colored::{Color, Colorize};

pub type Id = usize;

pub mod colors {
    use super::Color;

    pub const MAGIC: Color = Color::BrightMagenta;
    pub const VERSION: Color = Color::Yellow;
    pub const ATOM_KIND: Color = Color::Yellow;
    pub const NUM_CHILDREN: Color = Color::BrightRed;
    pub const KIND: Color = Color::Green;
    pub const LENGTH: Color = Color::BrightRed;
    pub const PAYLOAD: Color = Color::BrightCyan;
    pub const PADDING: Color = Color::Blue;
}

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

pub fn format_tree(children_left: &[usize], atom_starts: bool) -> String {
    children_left
        .iter()
        .enumerate()
        .map(|(index, left)| (index == children_left.len() - 1, left))
        .map(|(is_last_layer, left)| match left {
            0 => "  ",
            _ if !is_last_layer => "│ ",
            1 if atom_starts => "└─",
            _ if atom_starts => "├─",
            _ => "│ ",
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn format_atom_block_header(id: Id, kind: u64, num_children: u8) -> String {
    format!(
        "{}{}{}",
        format_atom_header_start("Block", id),
        format!("kind {} ({}), ", kind, kind_to_name(kind)).color(colors::KIND),
        format!(
            "{} {}",
            num_children,
            singular_or_plural(num_children.into(), "child", "children")
        )
        .color(colors::NUM_CHILDREN),
    )
}
pub fn format_atom_bytes_header(id: Id, length: usize) -> String {
    format!(
        "{}{}",
        format_atom_header_start("Bytes", id),
        format_n_bytes_long(length, false),
    )
}
pub fn format_atom_few_bytes_header(id: Id, length: usize, show_payload_label: bool) -> String {
    format!(
        "{}{}{}",
        format_atom_header_start("FewBytes", id),
        format_n_bytes_long(length, show_payload_label && length > 0),
        match show_payload_label {
            true => format_payload_label(length < 6),
            false => "".to_owned(),
        }
    )
}
pub fn format_atom_reference_header(id: Id, offset: u64) -> String {
    format!("{}{}", format_atom_header_start("Reference", id), offset)
}
fn format_atom_header_start(atom_kind: &str, id: Id) -> String {
    format!(
        "{}atom #{}, ",
        format!("{}, ", atom_kind).color(colors::ATOM_KIND).bold(),
        id,
    )
}
fn kind_to_name(kind: u64) -> String {
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
fn format_n_bytes_long(num_bytes: usize, trailing_comma: bool) -> String {
    format!(
        "{} {} long{}",
        num_bytes,
        singular_or_plural(num_bytes, "byte", "bytes"),
        match trailing_comma {
            true => ", ",
            false => "",
        }
    )
    .color(colors::LENGTH)
    .to_string()
}
pub fn format_payload_label(plus_padding: bool) -> String {
    format!(
        "{}{}",
        "Payload".color(colors::PAYLOAD),
        match plus_padding {
            true => " + padding".color(colors::PADDING).to_string(),
            false => "".to_owned(),
        }
    )
}

fn singular_or_plural(amount: usize, singular: &'static str, plural: &'static str) -> &'static str {
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
