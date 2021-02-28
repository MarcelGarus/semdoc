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
    fn try_from(bytes: &[u8]) -> Result<(MemoryMolecule, usize), MemoryError> {
        // TODO(marcelgarus): Handle AtomErrors correctly.
        let atom = Atom::try_from(bytes);
        println!("Bytes are {:?}", bytes);
        println!("Got atom {:?}", atom);
        match atom {
            Err(err) => Err(MemoryError::UnexpectedEnd), // TODO: Create proper error.
            Ok(atom) => Ok(match atom {
                Atom::Block { kind, num_children } => {
                    let mut children = vec![];
                    let mut cursor = 8;
                    for _ in 0..num_children {
                        match MemoryMolecule::try_from(&bytes[cursor..]) {
                            Ok((data, consumed_atoms)) => {
                                children.push(data);
                                cursor += consumed_atoms;
                            }
                            Err(_) => break,
                        }
                    }
                    while children.len() < num_children.into() {
                        children.push(Molecule::Error(MemoryError::UnexpectedEnd));
                    }
                    let data = Molecule::block(kind, children);
                    (data, cursor)
                }
                Atom::Reference(_offset) => {
                    todo!("Implement getting MemoryMolecule from Atom::Reference.")
                }
                Atom::Bytes(bytes) => (MemoryMolecule::Bytes(bytes), 1),
                Atom::FewBytes(bytes) => (MemoryMolecule::Bytes(bytes), 1),
            }),
        }
    }
}
