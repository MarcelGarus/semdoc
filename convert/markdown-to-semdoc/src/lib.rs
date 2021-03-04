use semdoc::{Block, Pure, SemDoc};
use comrak::{ComrakOptions, Arena, parse_document,  nodes::{AstNode, NodeValue, ListType}};

pub fn markdown_to_semdoc(markdown: &str) -> SemDoc<Pure> {
    let arena = Arena::new();

    let root = parse_document(
        &arena,
        markdown,
        &ComrakOptions::default(),);

    SemDoc::new(root.to_block())
}

trait ToBlock<'a> {
    fn to_block(&'a self) -> Block<Pure>;
}
impl<'a> ToBlock<'a> for AstNode<'a> {
    fn to_block(&'a self) -> Block<Pure> {
        use NodeValue::*;
        match self.data.borrow().value.clone() {
            Document => {
                Block::Paragraphs(self.children().collect::<Vec<_>>().clone().to_blocks())
            },
            Heading(_) => {
                Block::Section {
                    title: Box::new(Block::Paragraphs(self.children().collect::<Vec<_>>().clone().to_blocks())),
                    body: Box::new(Block::Empty),
                }
            },
            Paragraph => {
                Block::Flow(self.children().collect::<Vec<_>>().clone().to_blocks())
            },
            Text(text) => {
                Block::Text(String::from_utf8(text).unwrap())
            },
            SoftBreak => Block::Text(" ".to_owned()),
            Emph => {
                // TODO(marcelgarus): Handle emphasis.
                Block::Flow(self.children().collect::<Vec<_>>().clone().to_blocks())
            }
            Strong => {
                // TODO(marcelgarus): Handle strong text.
                Block::Flow(self.children().collect::<Vec<_>>().clone().to_blocks())
            }
            List(list) => {
                let items = self.children().collect::<Vec<_>>().clone().to_blocks();
                println!("List type is {:?}", list.list_type);
                match list.list_type {
                    ListType::Bullet => Block::BulletList(items),
                    ListType::Ordered => Block::OrderedList(items),
                }
            },
            Item(_) => {
                Block::Paragraphs(self.children().collect::<Vec<_>>().clone().to_blocks())
            },
            HtmlBlock(_) => {
                // TODO(marcelgarus): Handle HTML better.
                Block::Empty
            },
            ThematicBreak => Block::Empty,
            Link(link) => {
                // TODO(marcelgarus): Handle links better.
                Block::Text(String::from_utf8(link.title).unwrap())
            },
            _ => {
                println!("Not handling node {:?} yet.", self);
                Block::Empty
            }
        }
    }

}
trait ToBlocks {
    fn to_blocks(&self) -> Vec<Block<Pure>>;
}
impl<'a> ToBlocks for [&'a AstNode<'a>] {
    fn to_blocks(&self) -> Vec<Block<Pure> >{
        self.iter().map(|node| node.to_block()).collect()
    }

}
