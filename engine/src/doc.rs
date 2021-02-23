use std::convert::TryInto;

use super::blocks::*;
use super::molecules::*;
use super::scheduler::*;
use crate::atoms::*;

const MAGIC_BYTES: &[u8] = b"SemDoc";
const VERSION: u16 = 0;

#[derive(Debug)]
pub struct SemDoc {
    pub block: Block,
}
impl SemDoc {
    pub fn new(block: Block) -> Self {
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
                .schedule()
                .iter()
                .map(|atom| atom.to_bytes())
                .flatten()
                .collect::<Vec<_>>(),
        );
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        assert!(bytes.starts_with(MAGIC_BYTES));
        assert_eq!(u16::from_be_bytes(bytes[6..8].try_into().unwrap()), VERSION);
        let block = Block::from(&Molecule::from(&bytes[8..].parse_atoms().unwrap()).unwrap());
        Ok(Self { block })
    }
}
