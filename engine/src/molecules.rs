use super::atoms::*;

#[derive(Debug, Clone)]
pub struct Molecule {
    pub kind: u64,
    pub data: Vec<Data>,
}
#[derive(Debug, Clone)]
pub enum Data {
    Block(Molecule),
    Bytes(Vec<u8>),
}
impl Data {
    pub fn block(&self) -> Option<Molecule> {
        match self {
            Data::Block(molecule) => Some(molecule.clone()),
            _ => None,
        }
    }
    pub fn bytes(&self) -> Option<Vec<u8>> {
        match self {
            Data::Bytes(bytes) => Some(bytes.clone()),
            _ => None,
        }
    }
}

// Note: There serialization from `Molecule`s to `Atom` is slightly more
// complex, so it lives in its own module, the scheduler.

pub trait MoleculeFromAtoms {
    fn parse_molecule(&self) -> Result<Molecule, ()>;
}
impl MoleculeFromAtoms for [Atom] {
    fn parse_molecule(&self) -> Result<Molecule, ()> {
        // TODO(marcelgarus): Make sure all atoms were consumed?
        Ok(self.parse_data().unwrap().0.block().unwrap())
    }
}
trait MoleculeDataFromAtoms {
    fn parse_data(&self) -> Result<(Data, usize), ()>;
}
impl MoleculeDataFromAtoms for [Atom] {
    fn parse_data(&self) -> Result<(Data, usize), ()> {
        match self.first().unwrap() {
            Atom::Block { kind, num_children } => {
                let mut children = vec![];
                let mut cursor = 1;
                for _ in 0..*num_children {
                    let (data, consumed_atoms) = self[cursor..].parse_data().unwrap();
                    children.push(data);
                    cursor += consumed_atoms;
                }
                Ok((
                    Data::Block(Molecule {
                        kind: *kind,
                        data: children,
                    }),
                    cursor,
                ))
            }
            // TODO(marcelgarus): Reference.
            Atom::Bytes(bytes) => Ok((Data::Bytes(bytes.clone()), 1)),
            Atom::FewBytes(bytes) => Ok((Data::Bytes(bytes.clone()), 1)),
            _ => Err(()),
        }
    }
}
