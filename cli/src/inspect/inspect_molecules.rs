use colored::Colorize;
use semdoc::{Memory, Molecule, Source};

use super::utils::*;

mod colors {
    use colored::Color;

    pub const MOLECULE_KIND: Color = Color::Yellow;
    pub const BLOCK_KIND: Color = Color::Green;
}

pub fn inspect_molecules(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let molecule = Molecule::from(&bytes[8..]);

    println!(
        "{}",
        format_molecule::<Memory>(&molecule, terminal_width_or_80()),
    );
}

fn format_molecule<S: Source>(molecule: &Molecule<S>, width: usize) -> String {
    match molecule {
        Molecule::Block { kind, children } => format!(
            "{} {}\n{}",
            format_molecule_kind("Block"),
            format!("kind {} ({})", kind, kind_to_name(*kind)).color(colors::BLOCK_KIND),
            format_children_strings(
                &children
                    .iter()
                    .map(|block| format_molecule(&block, width - 2))
                    .collect::<Vec<_>>()[..],
            ),
        ),
        Molecule::Bytes(bytes) => format!(
            "{}:\n{}",
            format_molecule_kind("Bytes"),
            format_payload_bytes(&bytes, width)
        ),
        Molecule::Error(error) => format!("{}: {:?}", format_molecule_kind("Error"), error),
    }
}

fn format_molecule_kind(kind: &str) -> String {
    kind.color(colors::MOLECULE_KIND).bold().to_string()
}
