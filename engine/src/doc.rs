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
        let block = bytes
            .parse_atoms()
            .unwrap()
            .parse_molecules()
            .unwrap()
            .iter()
            .map(|molecule| molecule.higher())
            .collect::<Vec<_>>()
            .unflatten();
        Ok(Self { block })
    }
}
