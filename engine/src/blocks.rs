use crate::flatten::*;
use crate::molecules::*;

// TODO(marcelgarus): Document.
#[derive(Debug, Clone)]
pub enum Block {
    Unknown { kind: u64 },
    Empty,
    Text(String),
    Section { title: Box<Block>, body: Box<Block> },
    DenseSequence(Vec<Block>),
    SplitSequence(Vec<Block>),
}

pub trait Lowering {
    fn lower(&self) -> Molecule;
}
impl Lowering for FlatBlock {
    fn lower(&self) -> Molecule {
        use FlatBlock::*;

        // TODO(marcelgarus): Factor the branches out into functions defined below.
        match self {
            Unknown { kind } => todo!(
                "Can't turn FlatBlock::Unknown into Molecules (kind was {:?}).",
                kind
            ),
            Empty => Molecule {
                kind: 0,
                data: vec![],
            },
            Text(text) => Molecule {
                kind: 1,
                data: vec![MoleculeData::Bytes(text.as_bytes().to_vec())],
            },
            Section { title, body } => Molecule {
                kind: 2,
                data: vec![MoleculeData::Block(*title), MoleculeData::Block(*body)],
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
trait IdVecToData {
    fn lower(&self) -> Vec<MoleculeData>;
}
impl IdVecToData for Vec<Id> {
    fn lower(&self) -> Vec<MoleculeData> {
        self.iter()
            .map(|child| MoleculeData::Block(*child))
            .collect()
    }
}

pub trait Highering {
    fn higher(&self) -> FlatBlock;
}
impl Highering for Molecule {
    fn higher(&self) -> FlatBlock {
        // TODO(marcelgarus): Factor the branches out into functions defined below.
        match self.kind {
            0 => FlatBlock::Empty,
            1 => FlatBlock::Text(
                String::from_utf8(self.data.first().unwrap().bytes().unwrap()).unwrap(),
            ),
            2 => FlatBlock::Section {
                title: self.data.get(0).unwrap().block().unwrap(),
                body: self.data.get(1).unwrap().block().unwrap(),
            },
            3 => FlatBlock::DenseSequence(
                self.data.iter().map(|data| data.block().unwrap()).collect(),
            ),
            4 => FlatBlock::SplitSequence(
                self.data.iter().map(|data| data.block().unwrap()).collect(),
            ),
            _ => FlatBlock::Unknown { kind: self.kind },
        }
    }
}

// fn serialize_unknown(kind: usz)
