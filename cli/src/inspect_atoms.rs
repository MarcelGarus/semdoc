use colored::Colorize;
use semdoc_engine::atoms::*;
use semdoc_engine::memory::*;

use crate::utils::*;

pub fn inspect_atoms(file: &str) {
    println!("Inspecting atoms.");

    let bytes = std::fs::read(file).expect("File not found.");
    let mut cursor = 8;

    for _ in 0.. {
        let atom = match Atom::try_from(&bytes[cursor..]) {
            Ok(atom) => atom,
            Err(_) => {
                println!("Error.");
                return;
            }
        };
        println!("{}: {}", cursor, format_atom_header(cursor, &atom),);
        cursor += 8 * atom.length_in_words();
    }
}
fn format_atom_header(id: Id, atom: &Atom) -> String {
    match atom {
        Atom::Block { kind, num_children } => format_atom_block_header(id, *kind, *num_children),
        Atom::Bytes(bytes) => format_atom_bytes_header(id, bytes.len()),
        Atom::FewBytes(bytes) => format_atom_few_bytes_header(id, bytes.len(), false),
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
            byte.ascii_or_dot()
                .to_string()
                .color(colors::PADDING)
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("");
    format!("{}\n{}", hex, ascii)
}
