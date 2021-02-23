use super::atoms::*;

#[derive(Debug, Clone)]
pub struct Molecule {
    pub kind: u64,
    pub children: Vec<MoleculeData>,
}
impl Molecule {
    pub fn new(kind: u64, children: Vec<MoleculeData>) -> Self {
        Self { kind, children }
    }
    pub fn in_data(self) -> MoleculeData {
        MoleculeData::Block(self)
    }
}

#[derive(Debug, Clone)]
pub enum MoleculeData {
    Block(Molecule),
    Bytes(Vec<u8>),
}
impl MoleculeData {
    pub fn block(&self) -> Option<Molecule> {
        match self {
            MoleculeData::Block(molecule) => Some(molecule.clone()),
            _ => None,
        }
    }
    pub fn bytes(&self) -> Option<Vec<u8>> {
        match self {
            MoleculeData::Bytes(bytes) => Some(bytes.clone()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum MoleculeError {
    UnexpectedEnd,
    FirstAtomShouldBeBlock,
}

// Note: There serialization from `Molecule`s to `Atom` is slightly more
// complex, so it lives in its own module, the scheduler.

impl Molecule {
    pub fn from(atoms: &[Atom]) -> Result<Molecule, MoleculeError> {
        Ok(MoleculeData::from(atoms)?
            .0
            .block()
            .ok_or(MoleculeError::FirstAtomShouldBeBlock)?)
    }
}
impl MoleculeData {
    fn from(atoms: &[Atom]) -> Result<(MoleculeData, usize), MoleculeError> {
        Ok(match atoms.first().ok_or(MoleculeError::UnexpectedEnd)? {
            Atom::Block { kind, num_children } => {
                let mut children = vec![];
                let mut cursor = 1;
                for _ in 0..*num_children {
                    let (data, consumed_atoms) = MoleculeData::from(&atoms[cursor..])?;
                    children.push(data);
                    cursor += consumed_atoms;
                }
                let data = MoleculeData::Block(Molecule::new(*kind, children));
                (data, cursor)
            }
            Atom::Reference(offset) => {
                todo!("Implement getting MoleculeData from Atom::Reference.")
            }
            Atom::Bytes(bytes) => (MoleculeData::Bytes(bytes.clone()), 1),
            Atom::FewBytes(bytes) => (MoleculeData::Bytes(bytes.clone()), 1),
        })
    }
}
