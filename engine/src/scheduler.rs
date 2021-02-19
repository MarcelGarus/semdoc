use crate::atoms::Atom;
use crate::molecules;

pub trait Scheduler {
    fn schedule(&self) -> Vec<Atom>;
}
trait SchedulerInternal {
    fn _schedule(&self, output: &mut Vec<Atom>);
}
impl Scheduler for molecules::Molecule {
    fn schedule(&self) -> Vec<Atom> {
        let mut atoms = vec![];
        self._schedule(&mut atoms);
        atoms
    }
}
impl SchedulerInternal for molecules::Molecule {
    fn _schedule(&self, output: &mut Vec<Atom>) {
        output.push(Atom::Block {
            kind: self.kind,
            num_children: self.data.len() as u8, // TODO(marcelgarus): Handle overflow.
        });
        for data in &self.data {
            match data {
                molecules::Data::Block(molecule) => molecule._schedule(output),
                molecules::Data::Bytes(bytes) => {
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
