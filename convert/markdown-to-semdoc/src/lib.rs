use pulldown_cmark::{Event, Options, Parser, Tag};
use semdoc_engine::{Block, SemDoc};

pub fn markdown_to_semdoc(markdown: &str) -> SemDoc {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let mut parser = Parser::new_ext(markdown, options);
    let blocks = helper(&mut parser);
    SemDoc::new(Block::SplitSequence(blocks))
}

fn helper<'a, T: Iterator<Item = Event<'a>>>(events: &mut T) -> Vec<Block> {
    let mut blocks = vec![];
    loop {
        match events.next() {
            Some(event) => match event {
                Event::Start(tag) => match tag {
                    Tag::Heading(_) => blocks.push(Block::Section {
                        title: Box::new(Block::DenseSequence(helper(events))),
                        body: Box::new(Block::Empty),
                    }),
                    _ => blocks.extend_from_slice(&helper(events)),
                },
                Event::Text(text) => blocks.push(Block::Text(text.to_string())),
                Event::SoftBreak | Event::HardBreak => {}
                Event::End(_) => {
                    return blocks;
                }
                _ => println!("Unhandled event: {:?}", event),
            },
            None => return blocks,
        }
    }
}
