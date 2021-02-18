use super::blocks::*;

pub type Id = usize;

#[derive(Debug, Clone)]
pub enum FlatBlock {
    Unknown { kind: u64 },
    Empty,
    Text(String),
    Section { title: Id, body: Id },
    DenseSequence(Vec<Id>),
    SplitSequence(Vec<Id>),
}

pub trait Flatten {
    fn flatten(&self, next_id: Id) -> Vec<FlatBlock>;
}
// TODO(marcelgarus): Generate this impl automatically using macros.
impl Flatten for Block {
    fn flatten(&self, next_id: Id) -> Vec<FlatBlock> {
        use Block::*;

        match self {
            Unknown { kind } => {
                vec![FlatBlock::Unknown { kind: *kind }]
            }
            Empty => vec![FlatBlock::Empty],
            Text(text) => vec![FlatBlock::Text(text.clone())],
            Section { title, body } => {
                let title = title.flatten(next_id + 1);
                let body = body.flatten(next_id + 1 + title.len());
                let mut flat_blocks = vec![FlatBlock::Section {
                    title: next_id + 1,
                    body: next_id + 1 + title.len(),
                }];
                flat_blocks.append(&mut title.clone());
                flat_blocks.append(&mut body.clone());
                flat_blocks
            }
            DenseSequence(items) => {
                let (items, blocks) = items.flatten_all(next_id + 1);
                let mut flat_blocks = vec![FlatBlock::DenseSequence(items)];
                flat_blocks.append(&mut blocks.clone());
                flat_blocks
            }
            SplitSequence(items) => {
                let (items, blocks) = items.flatten_all(next_id + 1);
                let mut flat_blocks = vec![FlatBlock::SplitSequence(items)];
                flat_blocks.append(&mut blocks.clone());
                flat_blocks
            }
        }
    }
}
pub trait FlattenAll {
    fn flatten_all(&self, next_id: Id) -> (Vec<Id>, Vec<FlatBlock>);
}
impl FlattenAll for Vec<Block> {
    fn flatten_all(&self, next_id: Id) -> (Vec<Id>, Vec<FlatBlock>) {
        let mut id = next_id;
        let mut flat_blocks = vec![];
        let mut ids = vec![];
        for block in self {
            ids.push(id);
            let flattened = block.flatten(id);
            flat_blocks.append(&mut flattened.clone());
            id += flattened.len();
        }
        (ids, flat_blocks)
    }
}

pub trait Unflatten {
    fn unflatten(&self) -> Block;
    fn unflatten_at(&self, index: usize) -> Block;
}
// TODO(marcelgarus): Generate this impl automatically using macros.
impl Unflatten for [FlatBlock] {
    fn unflatten(&self) -> Block {
        self.unflatten_at(0)
    }
    fn unflatten_at(&self, index: usize) -> Block {
        use FlatBlock::*;

        match self.get(index).unwrap() {
            Unknown { kind } => Block::Unknown { kind: *kind },
            Empty => Block::Empty,
            Text(text) => Block::Text(text.clone()),
            Section { title, body } => Block::Section {
                title: Box::new(self.unflatten_at(*title)),
                body: Box::new(self.unflatten_at(*body)),
            },
            DenseSequence(items) => Block::DenseSequence(self.unflatten_all(items.clone())),
            SplitSequence(items) => Block::SplitSequence(self.unflatten_all(items.clone())),
        }
    }
}
pub trait UnflattenAll {
    fn unflatten_all(&self, ids: Vec<Id>) -> Vec<Block>;
}
impl UnflattenAll for [FlatBlock] {
    fn unflatten_all(&self, ids: Vec<Id>) -> Vec<Block> {
        ids.iter().map(|id| self.unflatten_at(*id)).collect()
    }
}
