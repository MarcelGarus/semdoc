use std::convert::TryInto;

use super::blocks::*;
use super::memory::*;
use crate::atoms::*;
use crate::source::*;

const MAGIC_BYTES: &[u8] = b"SemDoc";
const VERSION: u16 = 0;

#[derive(Debug, Clone)]
pub struct SemDoc<S: Source> {
    pub block: Block<S>,
}
impl<S: Source> SemDoc<S> {
    pub fn new(block: Block<S>) -> Self {
        Self { block }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(MAGIC_BYTES);
        bytes.extend_from_slice(&VERSION.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .block
                .to_molecule()
                .to_atoms()
                .iter()
                .map(|atom| atom.to_bytes())
                .flatten()
                .collect::<Vec<_>>(),
        );
        bytes
    }
}

pub fn from_bytes(bytes: &[u8]) -> SemDoc<Memory> {
    assert!(bytes.starts_with(MAGIC_BYTES));
    assert_eq!(u16::from_be_bytes(bytes[6..8].try_into().unwrap()), VERSION);
    let block = Block::from(&MemoryMolecule::from(&bytes[8..]));
    SemDoc { block }
}

#[derive(Debug, Clone)]
pub struct Pure();
impl Source for Pure {
    type Error = ();
}
pub type PureSemDoc = SemDoc<Pure>;
