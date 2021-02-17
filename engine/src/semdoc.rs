use std::ops::Range;

use super::atoms::*;
use super::book::{Block, *};
use super::utils::*;

pub type SemDoc = Block;

pub trait Serializable {
    fn deserialize(bytes: &[u8]) -> Result<SemDoc, ()>;
    fn serialize(&self, options: SerializationOptions) -> Vec<u8>;
}

pub struct SerializationOptions {
    pub inline_probability: f32,
}

impl Serializable for SemDoc {
    fn deserialize(bytes: &[u8]) -> Result<SemDoc, ()> {
        bytes.to_atom().unwrap().to_block()
    }
    fn serialize(&self, option: SerializationOptions) -> Vec<u8> {
        let atom = self.to_atom();
        atom.to_bytes()
    }
}

// trait FancyAtom: Sized {
//     fn len(&self) -> usize;
//     fn inlining(&self) -> Vec<(Self, f32)> {
//         self._inlining(0.0..1.0)
//     }
//     fn _inlining(&self, position: Range<f32>) -> Vec<(Self, f32)>;
// }
// impl<'a> FancyAtom for Atom<'a> {
//     fn len(&self) -> usize {
//         match self {
//             Atom::Block { children, .. } => {
//                 let children_len: usize = children.iter().map(|child| child.len()).sum();
//                 8 + children_len
//             }
//             Atom::Bytes(bytes) => 8 + bytes.len().round_up_to_multiple_of(8),
//         }
//     }
//     fn _inlining(&self, position: Range<f32>) -> Vec<(Self, f32)> {
//         0.0
//     }
// }
