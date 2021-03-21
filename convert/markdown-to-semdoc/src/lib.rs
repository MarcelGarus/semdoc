use comrak::{
    arena_tree::Children,
    nodes::{Ast, AstNode, ListType, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use semdoc::{Block, Pure, SemDoc};
use std::cell::RefCell;

pub fn markdown_to_semdoc(markdown: &str) -> SemDoc<Pure> {
    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &ComrakOptions::default());

    SemDoc::new(root.to_block())
}

trait ToBlock<'a> {
    fn to_block(&'a self) -> Block<Pure>;
}
impl<'a> ToBlock<'a> for AstNode<'a> {
    fn to_block(&'a self) -> Block<Pure> {
        use NodeValue::*;
        match self.data.borrow().value.clone() {
            Document => Block::Paragraphs(self.children().to_blocks()),
            Heading(_) => Block::Section {
                title: Box::new(Block::Paragraphs(self.children().to_blocks())),
                body: Box::new(Block::Empty),
            },
            Paragraph => Block::Flow(self.children().to_blocks()),
            Text(text) => Block::Text(String::from_utf8(text).unwrap()),
            SoftBreak => Block::Text(" ".to_owned()),
            // TODO(marcelgarus): Handle emphasis.
            Emph => Block::Flow(self.children().to_blocks()),
            // TODO(marcelgarus): Handle strong text.
            Strong => Block::Flow(self.children().to_blocks()),
            List(list) => {
                let items = self.children().to_blocks();
                println!("List type is {:?}", list.list_type);
                match list.list_type {
                    ListType::Bullet => Block::BulletList(items),
                    ListType::Ordered => Block::OrderedList(items),
                }
            }
            Item(_) => Block::Paragraphs(self.children().to_blocks()),
            // TODO(marcelgarus): Handle HTML better.
            HtmlBlock(_) => Block::Empty,
            ThematicBreak => Block::Empty,
            // TODO(marcelgarus): Handle links better.
            Link(link) => Block::Text(String::from_utf8(link.title).unwrap()),
            _ => {
                println!("Not handling node {:?} yet.", self);
                Block::Empty
            }
        }
    }
}
trait ToBlocks {
    fn to_blocks(self) -> Vec<Block<Pure>>;
}
impl<'a> ToBlocks for Vec<&'a AstNode<'a>> {
    fn to_blocks(self) -> Vec<Block<Pure>> {
        self.iter().map(|node| node.to_block()).collect()
    }
}
impl<'a> ToBlocks for Children<'a, RefCell<Ast>> {
    fn to_blocks(self) -> Vec<Block<Pure>> {
        self.collect::<Vec<_>>().clone().to_blocks()
    }
}
