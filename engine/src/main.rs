use std::fs::File;
use std::io::prelude::*;

mod engine;
use engine::atoms::Atom;

pub fn main() {
    let doc = Atom::Block {
        kind: vec![1, 1, 1],
        children: vec![
            Atom::Block {
                kind: vec![2, 1, 1],
                children: vec![Atom::Bytes(b"SemDoc".to_vec())],
            },
            Atom::Block {
                kind: vec![1, 2, 1],
                children: vec![
                    Atom::Block {
                        kind: vec![2, 1, 1],
                        children: vec![Atom::Bytes(b"Hello, world!".to_vec())],
                    },
                    Atom::Block {
                        kind: vec![2, 1, 1],
                        children: vec![Atom::Bytes(b"This is a test.".to_vec())],
                    },
                ],
            },
        ],
    };

    let bytes = doc.to_bytes();

    for chunk in bytes.chunks(8) {
        for i in 0..8 {
            print!("{:02x} ", chunk.get(i).unwrap());
        }
        println!();
    }

    let mut file = File::create("helloworld.sd").unwrap();
    file.write_all(&bytes).unwrap();

    let retrieved_doc = Atom::from(&mut bytes.into_iter());
    println!("Retrieved doc: {:?}", retrieved_doc);
}
