use std::fs::File;
use std::io::prelude::*;

mod atoms;
mod book;
mod semdoc;
mod utils;

use atoms::*;
use book::{Block::*, *};
use semdoc::*;

pub fn main() {
    let doc = Section {
        title: Box::new(Text("SemDoc".to_string())),
        body: Box::new(SplitSequence(vec![
            Text("Hello, world!".to_string()),
            Text("This is a test.".to_string()),
        ])),
    };
    let bytes = doc.serialize(SerializationOptions {
        inline_probability: 0.1,
    });

    for chunk in bytes.chunks(8) {
        for i in 0..8 {
            print!("{:02x} ", chunk.get(i).unwrap());
        }
        println!();
    }

    let mut file = File::create("helloworld.sd").unwrap();
    file.write_all(&bytes).unwrap();

    let doc = SemDoc::deserialize(&bytes[..]);
    println!("Retrieved doc: {:?}", doc);
}
