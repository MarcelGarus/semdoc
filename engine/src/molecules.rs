use super::atoms::*;
use super::flatten::*;

#[derive(Debug, Clone)]
pub struct Molecule {
    pub kind: u64,
    pub data: Vec<MoleculeData>,
}
#[derive(Debug, Clone)]
pub enum MoleculeData {
    Block(Id),
    Bytes(Vec<u8>),
}
impl MoleculeData {
    pub fn block(&self) -> Option<Id> {
        match self {
            MoleculeData::Block(id) => Some(*id),
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

pub trait MoleculesFromAtoms {
    fn parse_molecules(&self) -> Result<Vec<Molecule>, ()>;
}
impl MoleculesFromAtoms for Vec<Atom> {
    fn parse_molecules(&self) -> Result<Vec<Molecule>, ()> {
        let mut molecules = vec![];
        molecule_from_atoms(&self, &mut molecules).unwrap();
        // TODO(marcelgarus): Make sure all atoms were consumed?
        Ok(molecules)
    }
}
fn molecule_from_atoms(
    atoms: &[Atom],
    output: &mut Vec<Molecule>,
) -> Result<(MoleculeData, usize), ()> {
    match atoms.first().unwrap() {
        Atom::Block { kind, num_children } => {
            let id = output.len();
            output.push(Molecule {
                kind: *kind,
                data: vec![],
            });
            let mut cursor = 1;

            for _ in 0..*num_children {
                let (data, consumed_atoms) = molecule_from_atoms(&atoms[cursor..], output).unwrap();
                output[id].data.push(data);
                cursor += consumed_atoms;
            }
            Ok((MoleculeData::Block(id), cursor))
        }
        // TODO(marcelgarus): Reference.
        Atom::Bytes(bytes) => Ok((MoleculeData::Bytes(bytes.clone()), 1)),
        _ => Err(()),
    }
}
