use pulldown_cmark::{Event, Options, Parser, Tag};
use semdoc_engine::{Block, PureSemDoc};
use std::iter::Peekable;

pub fn markdown_to_semdoc(markdown: &str) -> PureSemDoc {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let mut parser = Parser::new_ext(markdown, options);
    // let block = parse_root(0, false, &mut parser.peekable());
    // SemDoc::new(block)
    todo!()
}

// fn parse_root<'a, I: Iterator<Item = Event<'a>>>(
//     current_level: u32,
//     is_inside_paragraph: bool,
//     events: &mut Peekable<I>,
// ) -> (Block, Option<u32>) {
//     println!("Parsing root at level {}", current_level);
//     let blocks = parse_multiple(current_level, is_inside_paragraph, events);
//     if is_inside_paragraph {
//         Block::DenseSequence(blocks)
//     } else {
//         Block::SplitSequence(blocks)
//     }
// }

// fn parse_multiple<'a, I: Iterator<Item = Event<'a>>>(
//     current_level: u32,
//     is_inside_paragraph: bool,
//     events: &mut Peekable<I>,
// ) -> (Vec<Block>, Option<u32>) {
//     let mut blocks = vec![];
//     loop {
//         match events.next() {
//             Some(event) => match event {
//                 Event::Start(tag) => match tag {
//                     Tag::Paragraph => {
//                         blocks.push(parse_root(current_level, true, events));
//                     }
//                     Tag::Heading(level) => {
//                         if level <= current_level {
//                             return (blocks, Some(level));
//                         }
//                         let title = parse_root(level, true, events);
//                         let content = parse_root(level, false, events);
//                         // blocks.push(Block::Section {
//                         //     title: Box::new(title),
//                         //     body: Box::new(content),
//                         // });
//                     }
//                     _ => {
//                         println!("Handling start of {:?}", tag);
//                         blocks.push(parse_root(current_level, is_inside_paragraph, events));
//                     }
//                 },
//                 Event::Text(text) => blocks.push(Block::Text(text.to_string())),
//                 Event::SoftBreak | Event::HardBreak => {}
//                 Event::End(_) => {
//                     return (blocks, None);
//                 }
//                 _ => println!("Unhandled event: {:?}", event),
//             },
//             None => return (blocks, None),
//         }
//     }
// }

// fn parse_section<'a, I: Iterator<Item = Event<'a>>>(
//     events: &mut Peekable<I>,
//     level: usize,
// ) -> Block {
//     let mut title = vec![];
//     let mut blocks = vec![];
//     match events.next() {
//         Event::Start(Heading(level)) {
//             parse_
//         }
//     }

//     loop {
//         match events.next() {
//             Some(event) => match event {
//                 Event::Start(tag) => match tag {
//                     Tag::Paragraph => {
//                         blocks.push(Block::Empty);
//                     }
//                     Tag::Heading(_) => blocks.push(Block::Section {
//                         title: Box::new(Block::DenseSequence(helper(events))),
//                         body: Box::new(Block::Empty),
//                     }),
//                     _ => blocks.extend_from_slice(&helper(events)),
//                 },
//                 Event::Text(text) => blocks.push(Block::Text(text.to_string())),
//                 Event::SoftBreak | Event::HardBreak => {}
//                 Event::End(_) => {
//                     return blocks;
//                 }
//                 _ => println!("Unhandled event: {:?}", event),
//             },
//             None => return blocks,
//         }
//     }
// }

// fn parse_content<'a, I: Iterator<Item = Event<'a>>>(
//     events: &mut Peekable<I>,
//     level: usize,
// ) -> Block {
// }
