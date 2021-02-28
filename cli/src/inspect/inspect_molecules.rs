use colored::Colorize;
use semdoc::{Memory, Molecule, Source};
use std::convert::TryInto;

use super::utils::*;

pub fn inspect_molecules(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let molecule = Molecule::from(&bytes[8..]);

    println!("{}", format_molecule::<Memory>(&molecule));
}

fn format_molecule<S: Source>(molecule: &Molecule<S>) -> String {
    match molecule {
        Molecule::Bytes(bytes) => format!("{}: {:?}", format_molecule_kind("Bytes"), bytes),
        Molecule::Block { kind, children } => format!(
            "{} {}\n{}",
            format_molecule_kind("Block,"),
            format_atom_block_header(0, *kind, children.len().try_into().unwrap()),
            format_children_without_roles(&children[..]),
        ),
        Molecule::Error(error) => format!("{}: {:?}", format_molecule_kind("Error"), error),
    }
}
fn format_molecule_kind(kind: &str) -> String {
    kind.yellow().bold().to_string()
}
fn format_children_without_roles<S: Source>(children: &[Molecule<S>]) -> String {
    format_children_strings(
        &children
            .iter()
            .map(|block| format_molecule(&block))
            .collect::<Vec<_>>()[..],
    )
}
