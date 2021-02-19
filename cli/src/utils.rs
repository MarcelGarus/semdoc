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
        format!("kind {}, ", kind).color(colors::KIND),
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
        format!("{} bytes long", length).color(colors::LENGTH),
    )
}
pub fn format_atom_few_bytes_header(id: Id, length: usize) -> String {
    format!(
        "{}{}",
        format_atom_header_start("FewBytes", id),
        format!("{} bytes long", length).color(colors::LENGTH),
    )
}
fn format_atom_header_start(atom_kind: &str, id: Id) -> String {
    format!(
        "{}atom #{}, ",
        format!("{}, ", atom_kind).color(colors::ATOM_KIND).bold(),
        id,
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
