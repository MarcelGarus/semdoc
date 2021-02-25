use super::atoms::*;
use super::molecule::*;
use super::source::*;

#[derive(Clone, Debug)]
pub struct Memory {}
impl Source for Memory {
    type Error = MemoryError;
}

#[derive(Debug, Clone)]
pub enum MemoryError {
    UnexpectedEnd,
}

pub type MemoryMolecule = Molecule<Memory>;
impl MemoryMolecule {
    pub fn from(atoms: &[Atom]) -> MemoryMolecule {
        match MemoryMolecule::try_from(atoms) {
            Ok((molecule, _)) => molecule,
            Err(error) => Molecule::Error(error),
        }
    }
}
impl MemoryMolecule {
    fn try_from(atoms: &[Atom]) -> Result<(MemoryMolecule, usize), MemoryError> {
        Ok(match atoms.first().ok_or(MemoryError::UnexpectedEnd)? {
            Atom::Block { kind, num_children } => {
                let mut children = vec![];
                let mut cursor = 1;
                for _ in 0..*num_children {
                    let (data, consumed_atoms) = MemoryMolecule::try_from(&atoms[cursor..])?;
                    children.push(data);
                    cursor += consumed_atoms;
                }
                let data = Molecule::block(*kind, children);
                (data, cursor)
            }
            Atom::Reference(_offset) => {
                todo!("Implement getting MemoryMolecule from Atom::Reference.")
            }
            Atom::Bytes(bytes) => (MemoryMolecule::Bytes(bytes.clone()), 1),
            Atom::FewBytes(bytes) => (MemoryMolecule::Bytes(bytes.clone()), 1),
        })
    }
}
