use super::blocks::Block;
use super::flatten::*;
use super::lowering::*;
use super::scheduler::*;

#[derive(Debug)]
pub struct SemDoc {
    block: Block,
}
impl SemDoc {
    pub fn new(block: Block) -> Self {
        Self { block }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.block
            .flatten(0)
            .iter()
            .map(|it| {
                println!("Block: {:?}", it);
                it
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|block| block.lower())
            .map(|it| {
                println!("Lowered: {:?}", it);
                it
            })
            .collect::<Vec<_>>()
            .schedule()
            .iter()
            .map(|it| {
                println!("Atom: {:?}", it);
                it
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|atom| atom.to_bytes())
            .map(|it| {
                println!("Bytes: {:?}", it);
                it
            })
            .flatten()
            .collect()
    }

    pub fn from_bytes(_bytes: &[u8]) -> Result<Self, ()> {
        // bytes.to_atom().unwrap().to_block()
        todo!()
    }
}
