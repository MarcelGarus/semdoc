use colored::Colorize;
use semdoc::Atom;

use super::utils::*;

pub fn inspect_atoms(file: &str) {
    println!("Inspecting atoms.");

    let bytes = std::fs::read(file).expect("File not found.");
    let mut cursor = 8;

    while cursor < bytes.len() {
        let atom = match Atom::try_from(&bytes[cursor..]) {
            Ok(atom) => atom,
            Err(error) => {
                println!("Error: {:?}", error);
                return;
            }
        };
        println!("{}: {}", cursor / 8, format_atom_header(cursor, &atom),);
        cursor += atom.length_in_bytes();
    }
}
fn format_atom_header(id: Id, atom: &Atom) -> String {
    match atom {
        Atom::Block { kind, num_children } => format_atom_block_header(id, *kind, *num_children),
        Atom::Bytes(bytes) => format!(
            "{}\n{}",
            format_atom_bytes_header(id, bytes.len()),
            format_bytes(bytes)
        ),
        Atom::FewBytes(bytes) => format!(
            "{}\n{}",
            format_atom_few_bytes_header(id, bytes.len(), false),
            format_bytes(bytes),
        ),
        Atom::Reference(offset) => format_atom_reference_header(id, *offset),
    }
}
fn format_bytes(bytes: &[u8]) -> String {
    let hex = bytes
        .iter()
        .map(|byte| format!("{:02x}", byte).color(colors::PAYLOAD).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let ascii = bytes
        .iter()
        .map(|byte| {
            byte.ascii_or_none()
                .unwrap_or('.')
                .to_string()
                .color(colors::PADDING)
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("");
    format!("{}\n{}", hex, ascii)
}
