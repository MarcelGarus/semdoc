use super::flatten::*;

#[derive(Debug, Clone)]
pub struct LoweredBlock {
    pub kind: u64,
    pub data: Vec<Data>,
}
#[derive(Debug, Clone)]
pub enum Data {
    Block(Id),
    Bytes(Vec<u8>),
}

pub trait Lowering {
    fn lower(&self) -> LoweredBlock;
}
impl Lowering for FlatBlock {
    fn lower(&self) -> LoweredBlock {
        use FlatBlock::*;

        // TODO(marcelgarus): Factor the branches out into functions defined below.
        match self {
            Unknown { kind, children } => LoweredBlock {
                kind: *kind,
                data: children.lower(),
            },
            Empty => LoweredBlock {
                kind: 0,
                data: vec![],
            },
            Text(text) => LoweredBlock {
                kind: 1,
                data: vec![Data::Bytes(text.as_bytes().to_vec())],
            },
            Section { title, body } => LoweredBlock {
                kind: 2,
                data: vec![Data::Block(*title), Data::Block(*body)],
            },
            DenseSequence(items) => LoweredBlock {
                kind: 3,
                data: items.lower(),
            },
            SplitSequence(items) => LoweredBlock {
                kind: 4,
                data: items.lower(),
            },
        }
    }
}
trait IdVecToData {
    fn lower(&self) -> Vec<Data>;
}
impl IdVecToData for Vec<Id> {
    fn lower(&self) -> Vec<Data> {
        self.iter().map(|child| Data::Block(*child)).collect()
    }
}
