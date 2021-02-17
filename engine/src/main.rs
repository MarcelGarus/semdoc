use std::fs::File;
use std::io::prelude::*;

mod book;
mod engine;

use book::{Block::*, *};
use engine::atoms::*;

pub fn main() {
    let doc = Section {
        title: Box::new(Text("SemDoc".to_string())),
        body: Box::new(SplitSequence(vec![
            Text("Hello, world!".to_string()),
            Text("This is a test.".to_string()),
        ])),
    };
    let atoms = doc.serialize();
    let bytes = atoms.to_bytes();

    for chunk in bytes.chunks(8) {
        for i in 0..8 {
            print!("{:02x} ", chunk.get(i).unwrap());
        }
        println!();
    }

    let mut file = File::create("helloworld.sd").unwrap();
    file.write_all(&bytes).unwrap();

    let retrieved_atoms = (&bytes[..]).to_atoms().unwrap();
    println!("Retrieved atoms: {:?}", retrieved_atoms);
    let retrieved_doc = retrieved_atoms.deserialize();
    println!("Retrieved doc: {:?}", retrieved_doc);
}
