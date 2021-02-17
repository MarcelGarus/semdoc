use std::convert::TryFrom;

pub type AtomKind = u64;

#[derive(Clone, Debug)]
pub enum Atom<'a> {
    Block {
        kind: AtomKind,
        children: Vec<Atom<'a>>,
    },
    Bytes(&'a [u8]),
}

pub trait ToAtoms {
    fn to_atom(&self) -> Result<Atom, ()>;
    fn to_atom_internal(&self) -> Result<(Atom, usize), ()>;
}
impl ToAtoms for [u8] {
    fn to_atom(&self) -> Result<Atom, ()> {
        match self.to_atom_internal() {
            Ok((child, _)) => Ok(child),
            Err(_) => Err(()),
        }
    }

    fn to_atom_internal<'a>(&'a self) -> Result<(Atom, usize), ()> {
        match self.first().unwrap() {
            0 => {
                let num_children = *self.get(1).unwrap();
                let kind = u64::clone_from_slice(&self[0..8]) & 0x00_00_ff_ff_ff_ff_ff_ff;

                let mut children: Vec<Atom<'a>> = vec![];
                let mut offset = 8;
                for _ in 0..num_children {
                    match (&self[offset..]).to_atom_internal() {
                        Ok((child, length)) => {
                            children.push(child);
                            offset += length;
                        }
                        Err(()) => return Err(()),
                    }
                }

                let atom = Atom::Block { kind, children };
                Ok((atom, offset))
            }
            1 => {
                let length =
                    (u64::clone_from_slice(&self[0..8]) & 0x00_ff_ff_ff_ff_ff_ff_ff) as usize;
                let payload_bytes = &self[8..(8 + length)];
                // TODO: Check alignment bytes.

                Ok((
                    Atom::Bytes(payload_bytes),
                    8 + length.round_up_to_multiple_of(8),
                ))
            }
            _ => Err(()),
        }
    }
}

impl<'a> Atom<'a> {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Atom::Block { kind, children } => {
                let mut bytes = vec![0];
                bytes.push(u8::try_from(children.len()).unwrap());
                bytes.extend_from_slice(&kind.to_be_bytes()[2..]);
                for child in children {
                    let child_bytes = child.to_bytes();
                    bytes.extend_from_slice(&child_bytes);
                }
                bytes
            }
            Atom::Bytes(payload_bytes) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(&[0u8; 6]);
                bytes.push(u8::try_from(payload_bytes.len()).unwrap());
                bytes.extend_from_slice(&payload_bytes);
                bytes.align();
                bytes
            }
        }
    }
}

trait Align {
    fn align(&mut self);
}
impl Align for Vec<u8> {
    fn align(&mut self) {
        let length = self.len();
        let filling_amount = 8 - length % 8;
        for _ in 0..filling_amount {
            self.push(0);
        }
    }
}

trait CloneFromSlice {
    fn clone_from_slice(bytes: &[u8]) -> Self;
}
impl CloneFromSlice for u64 {
    fn clone_from_slice(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 8);
        let mut tmp = [0u8; 8];
        tmp.clone_from_slice(&bytes);
        u64::from_be_bytes(tmp)
    }
}

trait RoundUpToMultipleOf {
    fn round_up_to_multiple_of(&self, number: Self) -> Self;
}
impl RoundUpToMultipleOf for usize {
    fn round_up_to_multiple_of(&self, number: Self) -> Self {
        let filling = number - self % number;
        self + filling
    }
}
