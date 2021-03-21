use crate::atoms::*;
use crate::molecule::*;
use crate::source::*;

#[derive(Clone, Debug)]
pub struct Memory {}
impl Source for Memory {
    type Error = MemoryError;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    UnexpectedEnd,
}

pub type MemoryMolecule = Molecule<Memory>;
impl MemoryMolecule {
    pub fn from(bytes: &[u8]) -> MemoryMolecule {
        match MemoryMolecule::try_from(bytes) {
            Ok((molecule, _)) => molecule,
            Err(error) => Molecule::Error(error),
        }
    }
}
impl MemoryMolecule {
    /// Tries to parse a `MemoryModule` from the given `bytes`.
    ///
    /// If successful, returns both the parsed `Module` and the number of bytes
    /// that were consumed.
    fn try_from(bytes: &[u8]) -> Result<(MemoryMolecule, usize), MemoryError> {
        match Atom::try_from(bytes) {
            // TODO: Create proper error based on the actual error that happened.
            Err(_) => Err(MemoryError::UnexpectedEnd),
            Ok(atom) => Ok(match atom {
                Atom::Block { kind, num_children } => {
                    let mut children = vec![];
                    let mut cursor = 8;
                    for _ in 0..num_children {
                        match MemoryMolecule::try_from(&bytes[cursor..]) {
                            Ok((data, consumed_bytes)) => {
                                children.push(data);
                                cursor += consumed_bytes;
                            }
                            Err(_) => break,
                        }
                    }
                    let data = Molecule::block(kind, children);
                    (data, cursor)
                }
                Atom::SmallBlock { kind, num_children } => {
                    let mut children = vec![];
                    let mut cursor = 8;
                    for _ in 0..num_children {
                        match MemoryMolecule::try_from(&bytes[cursor..]) {
                            Ok((data, consumed_bytes)) => {
                                children.push(data);
                                cursor += consumed_bytes;
                            }
                            Err(_) => break,
                        }
                    }
                    let data = Molecule::block(kind, children);
                    (data, cursor)
                }
                Atom::Reference(_offset) => {
                    todo!("Implement getting MemoryMolecule from Atom::Reference.")
                }
                Atom::Bytes(bytes) => {
                    let len = Atom::Bytes(bytes.clone()).length_in_bytes();
                    (MemoryMolecule::Bytes(bytes), len)
                }
                Atom::FewBytes(bytes) => {
                    let len = Atom::FewBytes(bytes.clone()).length_in_bytes();
                    (MemoryMolecule::Bytes(bytes), len)
                }
            }),
        }
    }
}
