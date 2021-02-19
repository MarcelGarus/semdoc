use colored::{Color, Colorize};
use semdoc_engine::flatten::Id;

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
            match num_children {
                1 => "child",
                _ => "children",
            }
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
        match num_bytes {
            1 => "byte",
            _ => "bytes",
        },
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

pub trait AsciiOrDot {
    fn ascii_or_dot(&self) -> char;
}
impl AsciiOrDot for u8 {
    fn ascii_or_dot(&self) -> char {
        if (32..=126).contains(self) {
            *self as char
        } else {
            '.'
        }
    }
}
