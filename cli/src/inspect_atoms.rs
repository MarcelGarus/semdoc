use colored::{Color, Colorize};
use semdoc_engine::atoms::*;
use semdoc_engine::flatten::Id;
use semdoc_engine::utils::*;

use crate::utils::*;

pub fn inspect_atoms(file: &str) {
    println!("Inspecting atoms.");

    let bytes = std::fs::read(file).expect("File not found.");
    let atoms = (&bytes[8..]).parse_atoms().expect("File corrupted.");
    let mut children_left = vec![]; // How many children are left in each indentation level.

    for (id, atom) in atoms.iter().enumerate() {
        println!(
            "{}{}",
            format_tree(&children_left, true),
            format_atom_header(id, &atom),
        );
        let num_layers = children_left.len();
        if num_layers > 0 && children_left[num_layers - 1] > 0 {
            children_left[num_layers - 1] -= 1;
        }

        // Draw the other lines.
        if let Atom::Bytes(bytes) | Atom::FewBytes(bytes) = &atom {
            let terminal_width = terminal_size::terminal_size()
                .map(|size| size.0 .0)
                .unwrap_or(80);
            let tree_prefix = format_tree(&children_left, false);
            let wrapped = textwrap::fill(
                &format_bytes(&bytes),
                terminal_width as usize - tree_prefix.len() - 1,
            );
            print!("{}", textwrap::indent(&wrapped, &tree_prefix));
        }

        if let Atom::Block { num_children, .. } = atom {
            children_left.push(*num_children as usize);
        }
        while matches!(children_left.last(), Some(left) if *left == 0) {
            children_left.pop();
        }
    }
}
fn format_atom_header(id: Id, atom: &Atom) -> String {
    match atom {
        Atom::Block { kind, num_children } => format_atom_block_header(id, *kind, *num_children),
        Atom::Bytes(bytes) => format_atom_bytes_header(id, bytes.len()),
        Atom::FewBytes(bytes) => format_atom_few_bytes_header(id, bytes.len(), false),
        Atom::Reference(offset) => panic!("Reference not formatable yet."),
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
