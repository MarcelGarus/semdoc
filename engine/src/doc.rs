use super::blocks::Block;
use super::blocks::*;
use super::flatten::*;
use super::molecules::*;
use super::scheduler::*;
use crate::atoms::*;

#[derive(Debug)]
pub struct SemDoc {
    block: Block,
}
impl SemDoc {
    pub fn new(block: Block) -> Self {
        Self { block }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.block
            .flatten(0)
            .iter()
            .map(|it| {
                println!("Block: {:?}", it);
                it
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|block| block.lower())
            .map(|it| {
                println!("Lowered: {:?}", it);
                it
            })
            .collect::<Vec<_>>()
            .schedule()
            .iter()
            .map(|it| {
                println!("Atom: {:?}", it);
                it
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|atom| atom.to_bytes())
            .map(|it| {
                println!("Bytes: {:?}", it);
                it
            })
            .flatten()
            .collect()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        // Parse atoms
        let atoms = {
            let mut atoms = vec![];
            let mut cursor = 0;
            while cursor < bytes.len() {
                let atom = Atom::from_bytes(&bytes[cursor..]).unwrap();
                cursor += 8 * atom.length_in_words();
                println!("Parsed atom {:?}. Cursor is now {}.", atom, cursor);
                atoms.push(atom);
            }
            atoms
        };
        println!("Atoms: {:?}", atoms);
        let molecules = atoms.parse_molecules().unwrap();
        println!("Molecules: {:?}", molecules);
        let flat_blocks: Vec<_> = molecules.iter().map(|molecule| molecule.higher()).collect();
        println!("Flat blocks: {:?}", flat_blocks);
        let block = flat_blocks.unflatten();
        println!("Block: {:?}", block);

        Ok(Self { block })
    }
}
