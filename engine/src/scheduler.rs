use crate::atoms::Atom;
use crate::lowering;
use crate::lowering::LoweredBlock;

pub trait Scheduler {
    fn schedule(&self) -> Vec<Atom>;
}
trait SchedulerInternal {
    fn _schedule(&self, index: usize, output: &mut Vec<Atom>);
}
impl Scheduler for Vec<LoweredBlock> {
    fn schedule(&self) -> Vec<Atom> {
        let mut atoms = vec![];
        self._schedule(0, &mut atoms);
        atoms
    }
}
impl SchedulerInternal for Vec<LoweredBlock> {
    fn _schedule(&self, index: usize, output: &mut Vec<Atom>) {
        let block = self.get(index).unwrap();
        output.push(Atom::Block {
            kind: block.kind,
            num_children: block.data.len() as u8, // TODO(marcelgarus): Handle overflow.
        });
        for data in &block.data {
            match data {
                lowering::Data::Block(id) => self._schedule(*id, output),
                lowering::Data::Bytes(bytes) => output.push(Atom::Bytes(bytes.clone())),
            }
        }
    }
}
