use colored::Colorize;
use semdoc_engine::{
    memory::{Memory, MemoryMolecule},
    molecule::Molecule,
    Source,
};

pub fn inspect_molecules(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let molecule = MemoryMolecule::from(&bytes[8..]);

    println!("{}", format_molecule::<Memory>(&molecule));
}

fn format_molecule<S: Source>(molecule: &Molecule<S>) -> String {
    match molecule {
        Molecule::Bytes(bytes) => format!("{}: {:?}", format_molecule_kind("Bytes"), bytes),
        Molecule::Block { kind, children } => format!(
            "{}\n{}",
            format_molecule_kind("Block"),
            format_children_without_roles(&children[..]),
        ),
        Molecule::Error(error) => format!("{}: {:?}", format_molecule_kind("Error"), error),
    }
}
fn format_molecule_kind(kind: &str) -> String {
    kind.yellow().bold().to_string()
}
fn format_children_with_roles<S: Source>(roles_and_children: Vec<(&str, &Molecule<S>)>) -> String {
    format_children_strings(
        &roles_and_children
            .iter()
            .map(|(role, block)| format!("{}: {}", role.green(), format_molecule(block)))
            .collect::<Vec<_>>()[..],
    )
}
fn format_children_without_roles<S: Source>(children: &[Molecule<S>]) -> String {
    format_children_strings(
        &children
            .iter()
            .map(|block| format_molecule(&block))
            .collect::<Vec<_>>()[..],
    )
}
fn format_children_strings(children: &[String]) -> String {
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
