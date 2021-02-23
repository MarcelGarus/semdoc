use crate::molecules::MoleculeData;
use crate::molecules::*;

// TODO(marcelgarus): Document.
#[derive(Debug, Clone)]
pub enum Block {
    // Special.
    Unknown { kind: u64 },
    Error,

    // General content.
    Empty,
    Text(String),
    Section { title: Box<Block>, body: Box<Block> },
    DenseSequence(Vec<Block>),
    SplitSequence(Vec<Block>),
}
use Block::*;

pub mod kinds {
    pub const EMPTY: u64 = 0;
    pub const TEXT: u64 = 1;
    pub const SECTION: u64 = 2;
    pub const DENSE_SEQUENCE: u64 = 3;
    pub const SPLIT_SEQUENCE: u64 = 4;
}

impl Block {
    pub fn to_molecule(&self) -> Molecule {
        match self {
            Unknown { kind } => todo!(
                "Can't turn Block::Unknown into Molecule yet (kind was {:?}).",
                kind
            ),
            Error => todo!("Can't turn Block::Error into Molecule yet."),
            Empty => lower_empty(),
            Text(text) => lower_text(text),
            Section { title, body } => lower_section(*title.clone(), *body.clone()),
            DenseSequence(items) => lower_dense_sequence(items.clone()),
            SplitSequence(items) => lower_split_sequence(items.clone()),
        }
    }

    pub fn from(molecule: &Molecule) -> Block {
        let heightened = match molecule.kind {
            kinds::EMPTY => heighten_empty(molecule.children.clone()),
            kinds::TEXT => heighten_text(molecule.children.clone()),
            kinds::SECTION => heighten_section(molecule.children.clone()),
            kinds::DENSE_SEQUENCE => heighten_dense_sequence(molecule.children.clone()),
            kinds::SPLIT_SEQUENCE => heighten_split_sequence(molecule.children.clone()),
            kind => return Unknown { kind },
        };
        match heightened {
            Ok(block) => block,
            Err(_) => Error,
        }
    }
}

#[derive(Debug)]
pub struct BlockError();

fn lower_empty() -> Molecule {
    Molecule::new(kinds::EMPTY, vec![])
}
fn heighten_empty(_: Vec<MoleculeData>) -> Result<Block, BlockError> {
    Ok(Block::Empty)
}

fn lower_text(text: &str) -> Molecule {
    Molecule::new(
        kinds::TEXT,
        vec![MoleculeData::Bytes(text.as_bytes().to_vec())],
    )
}
fn heighten_text(children: Vec<MoleculeData>) -> Result<Block, BlockError> {
    Ok(Text(
        String::from_utf8(
            children
                .first()
                .ok_or(BlockError())?
                .bytes()
                .ok_or(BlockError())?,
        )
        .map_err(|_| BlockError())?,
    ))
}

fn lower_section(title: Block, body: Block) -> Molecule {
    Molecule::new(
        kinds::SECTION,
        vec![
            MoleculeData::Block(title.to_molecule()),
            MoleculeData::Block(body.to_molecule()),
        ],
    )
}
fn heighten_section(children: Vec<MoleculeData>) -> Result<Block, BlockError> {
    Ok(Section {
        title: Box::new(Block::from(&children.get(0).unwrap().block().unwrap())),
        body: Box::new(Block::from(&children.get(1).unwrap().block().unwrap())),
    })
}

fn lower_dense_sequence(items: Vec<Block>) -> Molecule {
    Molecule::new(kinds::DENSE_SEQUENCE, items.into_molecules())
}
fn heighten_dense_sequence(children: Vec<MoleculeData>) -> Result<Block, BlockError> {
    Ok(DenseSequence(
        children
            .iter()
            .map(|data| Block::from(&data.block().unwrap()))
            .collect(),
    ))
}

fn lower_split_sequence(items: Vec<Block>) -> Molecule {
    Molecule::new(kinds::SPLIT_SEQUENCE, items.into_molecules())
}
fn heighten_split_sequence(children: Vec<MoleculeData>) -> Result<Block, BlockError> {
    Ok(SplitSequence(
        children
            .iter()
            .map(|data| Block::from(&data.block().unwrap()))
            .collect(),
    ))
}

trait LowerMultiple {
    fn into_molecules(self) -> Vec<MoleculeData>;
}
impl LowerMultiple for Vec<Block> {
    fn into_molecules(self) -> Vec<MoleculeData> {
        self.iter()
            .map(|child| MoleculeData::Block(child.to_molecule()))
            .collect()
    }
}
