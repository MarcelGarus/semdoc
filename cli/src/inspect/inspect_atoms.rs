use colored::Colorize;
use semdoc::Atom;

use super::utils::*;

pub mod colors {
    use colored::Color;

    pub const ATOM_KIND: Color = Color::Yellow;
    pub const BLOCK_KIND: Color = Color::Green;
    pub const NUM_CHILDREN: Color = Color::BrightRed;
    pub const LENGTH: Color = Color::BrightRed;
}

pub fn inspect_atoms(file: &str) {
    println!("Inspecting atoms.");

    let bytes = std::fs::read(file).expect("File not found.");
    let mut cursor = 8;

    println!("{:4}  {}", "Word".bold(), "Atom".bold(),);
    while cursor < bytes.len() {
        let atom = match Atom::try_from(&bytes[cursor..]) {
            Ok(atom) => atom,
            Err(error) => {
                println!("Error: {:?}", error);
                return;
            }
        };
        println!(
            "{:4}  {}",
            cursor / 8,
            format_atom(&atom, terminal_width_or_80()),
        );
        cursor += atom.length_in_bytes();
    }
}

fn format_atom(atom: &Atom, width: usize) -> String {
    match atom {
        Atom::Block { kind, num_children } => format!(
            "{}{}{}",
            format_atom_type("Block"),
            format!("kind {} ({}), ", kind, kind_to_name(*kind)).color(colors::BLOCK_KIND),
            format_n_children(*num_children as usize),
        ),
        Atom::SmallBlock { kind, num_children } => {
            format!(
                "{}{}{}",
                format_atom_type("SmallBlock"),
                format!("kind {} ({}), ", kind, kind_to_name(*kind)).color(colors::BLOCK_KIND),
                format_n_children(*num_children as usize).color(colors::NUM_CHILDREN),
            )
        }
        Atom::Bytes(bytes) => format!(
            "{}\n{}",
            format!(
                "{}{}",
                format_atom_type("Bytes"),
                format_n_bytes_long(bytes.len(), false),
            ),
            format_bytes(bytes, width),
        ),
        Atom::FewBytes(bytes) => format!(
            "{}\n{}",
            format!(
                "{}{}",
                format_atom_type("FewBytes"),
                format_n_bytes_long(bytes.len(), bytes.len() > 0),
            ),
            format_bytes(&bytes, width),
        ),
        Atom::Reference(offset) => {
            format!("{}{}", format_atom_type("Reference"), offset)
        }
    }
}

fn format_atom_type(atom_type: &str) -> String {
    format!("{}, ", atom_type)
        .color(colors::ATOM_KIND)
        .bold()
        .to_string()
}

fn format_n_children(num_children: usize) -> String {
    format!(
        "{} {}",
        num_children,
        singular_or_plural(num_children, "child", "children")
    )
    .color(colors::NUM_CHILDREN)
    .to_string()
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

fn format_bytes(bytes: &[u8], width: usize) -> String {
    format_payload_bytes(bytes, width - 6)
        .split("\n")
        .map(|line| format!("      {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}
