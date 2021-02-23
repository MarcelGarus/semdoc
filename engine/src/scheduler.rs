use crate::atoms::*;
use crate::molecules::*;

pub trait Scheduler {
    fn schedule(&self) -> Vec<Atom>;
}
trait SchedulerInternal {
    fn _schedule(&self, output: &mut Vec<Atom>);
}
impl Scheduler for Molecule {
    fn schedule(&self) -> Vec<Atom> {
        let mut atoms = vec![];
        self._schedule(&mut atoms);
        atoms
    }
}
impl SchedulerInternal for Molecule {
    fn _schedule(&self, output: &mut Vec<Atom>) {
        output.push(Atom::Block {
            kind: self.kind,
            num_children: self.children.len() as u8, // TODO(marcelgarus): Handle overflow.
        });
        for data in &self.children {
            match data {
                MoleculeData::Block(molecule) => molecule._schedule(output),
                MoleculeData::Bytes(bytes) => {
                    if bytes.len() < 256 {
                        output.push(Atom::FewBytes(bytes.clone()));
                    } else {
                        output.push(Atom::Bytes(bytes.clone()));
                    }
                }
            }
        }
    }
}
