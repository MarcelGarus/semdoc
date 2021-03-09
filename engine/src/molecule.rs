use crate::atoms::*;
use crate::source::*;

#[derive(Debug, Clone)]
pub enum Molecule<S: Source> {
    Bytes(Vec<u8>),
    Block {
        kind: u64,
        children: Vec<Molecule<S>>,
    },
    Error(S::Error),
}
impl<S: Source> Molecule<S> {
    pub fn block(kind: u64, children: Vec<Molecule<S>>) -> Self {
        Self::Block { kind, children }
    }
}

impl<S: Source> Molecule<S> {
    pub fn to_atoms(&self) -> Vec<Atom> {
        let mut atoms = vec![];
        self.to_atoms_into(&mut atoms);
        atoms
    }

    // TODO: Use Atom::Reference when appropriate.
    fn to_atoms_into(&self, output: &mut Vec<Atom>) {
        match self {
            Molecule::Block { kind, children } => {
                output.push(if children.len() < 256 {
                    Atom::SmallBlock {
                        kind: *kind,
                        num_children: children.len() as u8,
                    }
                } else {
                    Atom::Block {
                        kind: *kind,
                        num_children: children.len() as u64,
                    }
                });
                for child in children {
                    child.to_atoms_into(output);
                }
            }
            Molecule::Bytes(bytes) => output.push(if bytes.len() < 256 {
                Atom::FewBytes(bytes.clone())
            } else {
                Atom::Bytes(bytes.clone())
            }),
            Molecule::Error(_) => todo!("Handle error while serializing molecule into atoms."),
        }
    }
}
