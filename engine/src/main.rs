use std::fs::File;
use std::io::prelude::*;

mod atoms;
mod blocks;
mod doc;
mod molecules;
mod scheduler;
mod utils;

use blocks::Block::*;
use doc::SemDoc;

pub fn main() {
    let doc = SemDoc::new(Section {
        title: Box::new(Text("SemDoc".to_string())),
        body: Box::new(SplitSequence(vec![
            Text("Hello, world!".to_string()),
            Text("This is a test. Hello!".to_string()),
        ])),
    });
    let bytes = doc.to_bytes();

    for chunk in bytes.chunks(8) {
        for i in 0..8 {
            print!("{:02x} ", chunk.get(i).unwrap());
        }
        println!();
    }

    let mut file = File::create("helloworld.sd").unwrap();
    file.write_all(&bytes).unwrap();

    let doc = SemDoc::from_bytes(&bytes[..]);
    println!("Retrieved doc: {:?}", doc);
}
