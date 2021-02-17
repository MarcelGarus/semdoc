// TODO(marcelgarus): Document.
#[derive(Debug)]
pub enum Block {
    Unknown { kind: u64, children: Vec<Block> },
    Empty,
    Text(String),
    Section { title: Box<Block>, body: Box<Block> },
    DenseSequence(Vec<Block>),
    SplitSequence(Vec<Block>),
}
