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
            .map(|block| block.lower())
            .collect::<Vec<_>>()
            .schedule()
            .iter()
            .map(|atom| atom.to_bytes())
            .flatten()
            .collect()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        let atoms = {
            let mut atoms = vec![];
            let mut cursor = 0;
            while cursor < bytes.len() {
                let atom = Atom::from_bytes(&bytes[cursor..]).unwrap();
                cursor += 8 * atom.length_in_words();
                atoms.push(atom);
            }
            atoms
        };
        let molecules = atoms.parse_molecules().unwrap();
        let flat_blocks: Vec<_> = molecules.iter().map(|molecule| molecule.higher()).collect();
        let block = flat_blocks.unflatten();

        Ok(Self { block })
    }
}
