use colored::Colorize;
use semdoc_engine::{from_bytes, Block, SemDoc, Source};

pub fn inspect_blocks(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let doc = from_bytes(&bytes);

    println!("{}", format_block(&doc.block));
}

fn format_block<S: Source>(block: &Block<S>) -> String {
    use Block::*;

    match block {
        Error(_) => format_block_kind("Error"),
        // Unknown { .. } => format_block_kind("Unknown"),
        Empty => format_block_kind("Empty"),
        Text(text) => format!("{}: {}", format_block_kind("Text"), text),
        Section { title, body } => format!(
            "{}\n{}",
            format_block_kind("Section"),
            format_children_with_roles(vec![("title", title), ("body", body)]),
        ),
        DenseSequence(items) => format!(
            "{}\n{}",
            format_block_kind("DenseSequence"),
            format_children_without_roles(&items[..]),
        ),
        SplitSequence(items) => format!(
            "{}\n{}",
            format_block_kind("SplitSequence"),
            format_children_without_roles(&items[..]),
        ),
    }
}
fn format_block_kind(kind: &str) -> String {
    kind.yellow().bold().to_string()
}
fn format_children_with_roles<S: Source>(roles_and_children: Vec<(&str, &Block<S>)>) -> String {
    format_children_strings(
        &roles_and_children
            .iter()
            .map(|(role, block)| format!("{}: {}", role.green(), format_block(block)))
            .collect::<Vec<_>>()[..],
    )
}
fn format_children_without_roles<S: Source>(children: &[Block<S>]) -> String {
    format_children_strings(
        &children
            .iter()
            .map(|block| format_block(&block))
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
