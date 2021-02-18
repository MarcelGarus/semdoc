use std::convert::TryFrom;

use super::utils::*;

pub type AtomKind = u64;

// TODO(marcelgarus): Add atom for packed bytes.
#[derive(Clone, Debug)]
pub enum Atom {
    Block { kind: AtomKind, num_children: u8 },
    Reference(u64),
    Bytes(Vec<u8>),
    FewBytes(Vec<u8>),
}

pub trait LengthInWords {
    fn length_in_words(&self) -> usize;
}
impl LengthInWords for Atom {
    fn length_in_words(&self) -> usize {
        use Atom::*;

        match self {
            Block { .. } => 1,
            Reference(_) => 1,
            Bytes(bytes) => 1 + bytes.len().round_up_to_multiple_of(8) / 8,
            FewBytes(bytes) => {
                1 + (if bytes.len() < 6 { 0 } else { bytes.len() - 6 }).round_up_to_multiple_of(8)
                    / 8
            }
        }
    }
}

impl Atom {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Atom::Block { num_children, kind } => {
                let mut bytes = vec![0];
                bytes.push(*num_children);
                bytes.extend_from_slice(&kind.to_be_bytes()[2..]);
                bytes
            }
            Atom::Reference(offset) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(&offset.to_be_bytes()[1..]);
                bytes
            }
            Atom::Bytes(payload_bytes) => {
                let mut bytes = vec![2];
                bytes.extend_from_slice(
                    &u64::try_from(payload_bytes.len()).unwrap().to_be_bytes()[1..],
                );
                bytes.extend_from_slice(&payload_bytes);
                bytes.align();
                bytes
            }
            Atom::FewBytes(payload_bytes) => {
                let mut bytes = vec![3];
                bytes.push(payload_bytes.len() as u8);
                bytes.extend_from_slice(&payload_bytes);
                bytes.align();
                bytes
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Atom, ()> {
        match bytes.first().unwrap() {
            0 => Ok(Atom::Block {
                kind: u64::clone_from_slice(&bytes[0..8]) & 0x00_00_ff_ff_ff_ff_ff_ff,
                num_children: *bytes.get(1).unwrap(),
            }),
            1 => {
                let offset = u64::clone_from_slice(&bytes[0..8]) & 0x00_ff_ff_ff_ff_ff_ff_ff;
                Ok(Atom::Reference(offset))
            }
            2 => {
                let length =
                    (u64::clone_from_slice(&bytes[0..8]) & 0x00_ff_ff_ff_ff_ff_ff_ff) as usize;
                let payload_bytes = &bytes[8..(8 + length)];
                // TODO(marcelgarus): Check alignment bytes.
                Ok(Atom::Bytes(payload_bytes.to_vec()))
            }
            3 => {
                let length = bytes[1] as usize;
                let payload_bytes = &bytes[2..(2 + length)];
                // TODO(marcelgarus): Check alignment bytes.
                Ok(Atom::FewBytes(payload_bytes.to_vec()))
            }
            kind => todo!("Unknown atom kind {:02x}. Bytes: {:?}", kind, bytes),
        }
    }
}

pub trait ParseAtoms {
    fn parse_atoms(&self) -> Result<Vec<Atom>, ()>;
}
impl ParseAtoms for [u8] {
    fn parse_atoms(&self) -> Result<Vec<Atom>, ()> {
        let mut atoms = vec![];
        let mut cursor = 0;
        while cursor < self.len() {
            let atom = Atom::from_bytes(&self[cursor..]).unwrap();
            println!("Parsed {:?} Length is {}", atom, atom.length_in_words());
            cursor += 8 * atom.length_in_words();
            atoms.push(atom);
        }
        Ok(atoms)
    }
}
