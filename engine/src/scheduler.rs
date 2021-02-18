use crate::atoms::Atom;
use crate::molecules::*;

pub trait Scheduler {
    fn schedule(&self) -> Vec<Atom>;
}
trait SchedulerInternal {
    fn _schedule(&self, index: usize, output: &mut Vec<Atom>);
}
impl Scheduler for Vec<Molecule> {
    fn schedule(&self) -> Vec<Atom> {
        let mut atoms = vec![];
        self._schedule(0, &mut atoms);
        atoms
    }
}
impl SchedulerInternal for Vec<Molecule> {
    fn _schedule(&self, index: usize, output: &mut Vec<Atom>) {
        let block = self.get(index).unwrap();
        output.push(Atom::Block {
            kind: block.kind,
            num_children: block.data.len() as u8, // TODO(marcelgarus): Handle overflow.
        });
        for data in &block.data {
            match data {
                MoleculeData::Block(id) => self._schedule(*id, output),
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
