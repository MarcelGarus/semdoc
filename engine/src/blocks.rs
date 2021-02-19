use crate::molecules;
use crate::molecules::Molecule;

// TODO(marcelgarus): Document.
#[derive(Debug, Clone)]
pub enum Block {
    // Special.
    Unknown { kind: u64 },

    // Content.
    Empty,
    Text(String),
    Section { title: Box<Block>, body: Box<Block> },
    DenseSequence(Vec<Block>),
    SplitSequence(Vec<Block>),
}

pub trait Lowering {
    fn lower(&self) -> Molecule;
}
impl Lowering for Block {
    fn lower(&self) -> Molecule {
        use super::blocks::Block::*;
        use molecules::Data::*;

        // TODO(marcelgarus): Factor the branches out into functions defined below.
        match self {
            Unknown { kind } => todo!(
                "Can't turn Block::Unknown into Molecules (kind was {:?}).",
                kind
            ),
            Empty => Molecule {
                kind: 0,
                data: vec![],
            },
            Text(text) => Molecule {
                kind: 1,
                data: vec![Bytes(text.as_bytes().to_vec())],
            },
            Section { title, body } => Molecule {
                kind: 2,
                data: vec![Block(title.lower()), Block(body.lower())],
            },
            DenseSequence(items) => Molecule {
                kind: 3,
                data: items.lower(),
            },
            SplitSequence(items) => Molecule {
                kind: 4,
                data: items.lower(),
            },
        }
    }
}
trait BlocksLowering {
    fn lower(&self) -> Vec<molecules::Data>;
}
impl BlocksLowering for [Block] {
    fn lower(&self) -> Vec<molecules::Data> {
        self.iter()
            .map(|child| molecules::Data::Block(child.lower()))
            .collect()
    }
}

pub trait Highering {
    fn higher(&self) -> Block;
}
impl Highering for Molecule {
    fn higher(&self) -> Block {
        use Block::*;

        // TODO(marcelgarus): Factor the branches out into functions defined below.
        match self.kind {
            0 => Empty,
            1 => Text(String::from_utf8(self.data.first().unwrap().bytes().unwrap()).unwrap()),
            2 => Section {
                title: Box::new(self.data.get(0).unwrap().block().unwrap().higher()),
                body: Box::new(self.data.get(1).unwrap().block().unwrap().higher()),
            },
            3 => DenseSequence(
                self.data
                    .iter()
                    .map(|data| data.block().unwrap().higher())
                    .collect(),
            ),
            4 => SplitSequence(
                self.data
                    .iter()
                    .map(|data| data.block().unwrap().higher())
                    .collect(),
            ),
            _ => Unknown { kind: self.kind },
        }
    }
}
