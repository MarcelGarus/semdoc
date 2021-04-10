use colored::Colorize;
use semdoc::{Block, SemDoc, Source};

use super::utils::*;

pub fn inspect_blocks(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let doc = SemDoc::from_bytes(&bytes).unwrap();

    print!("{}", format_block(&doc.block, terminal_width_or_80(), 0));
}

fn format_block<S: Source>(block: &Block<S>, width: usize, offset: usize) -> String {
    use Block::*;

    match block {
        Error(error) => format!("{}: {:?}", format_block_kind("Error"), error),
        Empty => format_block_kind("Empty"),
        Text(text) => format!(
            "{}{}",
            format_block_kind("Text: "),
            textwrap::wrap(
                text,
                textwrap::Options::new(width).initial_indent(&indent(offset + 6))
            )
            .join("\n")[(offset + 6)..]
                .to_owned()
        ),
        Section { title, body } => format!(
            "{}\n{}",
            format_block_kind("Section"),
            format_children_with_roles(vec![("title", title), ("body", body)], width),
        ),
        Flow(children) => format!(
            "{}\n{}",
            format_block_kind("Flow"),
            format_children_without_roles(&children[..], width),
        ),
        Paragraphs(children) => format!(
            "{}\n{}",
            format_block_kind("Paragraphs"),
            format_children_without_roles(&children[..], width),
        ),
        BulletList(items) => format!(
            "{}\n{}",
            format_block_kind("BulletList"),
            format_children_without_roles(&items[..], width),
        ),
        OrderedList(items) => format!(
            "{}\n{}",
            format_block_kind("OrderedList"),
            format_children_without_roles(&items[..], width),
        ),
    }
}

fn format_block_kind(kind: &str) -> String {
    kind.yellow().bold().to_string()
}

fn format_children_with_roles<S: Source>(
    roles_and_children: Vec<(&str, &Block<S>)>,
    width: usize,
) -> String {
    format_children_strings(
        &roles_and_children
            .iter()
            .map(|(role, block)| {
                format!(
                    "{}{}",
                    format!("{}: ", role).green(),
                    format_block(block, width - 2, role.len() + 2)
                )
            })
            .collect::<Vec<_>>()[..],
    )
}

fn format_children_without_roles<S: Source>(children: &[Block<S>], width: usize) -> String {
    format_children_strings(
        &children
            .iter()
            .map(|block| format_block(&block, width - 2, 0))
            .collect::<Vec<_>>()[..],
    )
}

fn indent(amount: usize) -> String {
    std::iter::repeat(" ").take(amount).collect::<String>()
}
