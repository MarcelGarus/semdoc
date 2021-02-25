use crate::molecule::*;
use crate::source::*;

// TODO(marcelgarus): Document.
#[derive(Debug, Clone)]
pub enum Block<S: Source> {
    // Implementation-specific.
    Error(Error<S>),

    // General content.
    Empty,
    Text(String),
    Section {
        title: Box<Block<S>>,
        body: Box<Block<S>>,
    },
    DenseSequence(Vec<Block<S>>),
    SplitSequence(Vec<Block<S>>),
}
use Block::*;

#[derive(Debug, Clone)]
pub enum Error<S: Source> {
    BlockLayer(BlockError),
    LowerLayer(S::Error),
}

pub mod kinds {
    pub const EMPTY: u64 = 0;
    pub const TEXT: u64 = 1;
    pub const SECTION: u64 = 2;
    pub const DENSE_SEQUENCE: u64 = 3;
    pub const SPLIT_SEQUENCE: u64 = 4;
}

impl<S: Source> Block<S> {
    pub fn to_molecule(&self) -> Molecule<S> {
        match self {
            Error(_) => todo!("Can't turn an Error into a Molecule yet."),
            Empty => Molecule::block(kinds::EMPTY, vec![]),
            Text(text) => {
                Molecule::block(kinds::TEXT, vec![Molecule::Bytes(text.as_bytes().to_vec())])
            }
            Section { title, body } => Molecule::block(
                kinds::SECTION,
                vec![title.to_molecule(), body.to_molecule()],
            ),
            DenseSequence(items) => {
                Molecule::block(kinds::DENSE_SEQUENCE, items.clone().into_molecules())
            }
            SplitSequence(items) => {
                Molecule::block(kinds::SPLIT_SEQUENCE, items.clone().into_molecules())
            }
        }
    }

    pub fn try_from(kind: u64, children: Vec<Molecule<S>>) -> Result<Block<S>, BlockError> {
        match kind {
            kinds::EMPTY => Ok(Block::Empty),
            kinds::TEXT => Ok(Text(
                String::from_utf8(children.need_at(0)?.need_bytes()?)
                    .map_err(|_| BlockError::InvalidUtf8Encoding)?,
            )),
            kinds::SECTION => Ok(Section {
                title: Box::new(Block::from(&children.need_at(0)?)),
                body: Box::new(Block::from(&children.need_at(1)?)),
            }),
            kinds::DENSE_SEQUENCE => Ok(DenseSequence(
                children.iter().map(|data| Block::from(data)).collect(),
            )),
            kinds::SPLIT_SEQUENCE => Ok(SplitSequence(
                children.iter().map(|data| Block::from(data)).collect(),
            )),
            _kind => Err(BlockError::UnknownKind),
        }
    }

    pub fn from(molecule: &Molecule<S>) -> Block<S> {
        match molecule {
            Molecule::Bytes(_) => Error(Error::BlockLayer(BlockError::ExpectedBlock)),
            Molecule::Error(error) => Error(Error::LowerLayer(error.clone())),
            Molecule::Block { kind, children } => match Self::try_from(*kind, children.clone()) {
                Ok(block) => block,
                Err(error) => Block::Error(Error::BlockLayer(error)),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockError {
    ExpectedBlock,
    ExpectedBytes,
    UnknownKind,
    InvalidUtf8Encoding,
    TooFewMolecules,
}

trait IntoMolecules<S: Source> {
    fn into_molecules(self) -> Vec<Molecule<S>>;
}
impl<S: Source> IntoMolecules<S> for Vec<Block<S>> {
    fn into_molecules(self) -> Vec<Molecule<S>> {
        self.iter().map(|child| child.to_molecule()).collect()
    }
}
trait NeedAt<S: Source> {
    fn need_at(&self, index: usize) -> Result<Molecule<S>, BlockError>;
}
impl<S: Source> NeedAt<S> for Vec<Molecule<S>> {
    fn need_at(&self, index: usize) -> Result<Molecule<S>, BlockError> {
        Ok(self.get(index).ok_or(BlockError::TooFewMolecules)?.clone())
    }
}
trait NeedBytes<S: Source> {
    fn need_bytes(&self) -> Result<Vec<u8>, BlockError>;
}
impl<S: Source> NeedBytes<S> for Molecule<S> {
    fn need_bytes(&self) -> Result<Vec<u8>, BlockError> {
        match self {
            Molecule::Bytes(bytes) => Ok(bytes.clone()),
            _ => Err(BlockError::ExpectedBytes),
        }
    }
}
