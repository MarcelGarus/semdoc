use crate::molecule::*;
use crate::source::*;

/// Every SemDoc is a composition of blocks.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Block<S: Source> {
    // Implementation-specific.

    /// Indicates that an error occurred.
    ///
    /// Usually, this block is not intentionally created by users. Instead, the engine creates it if
    /// some data couldn't be parsed. The contained error is one of these:
    ///
    /// * An error of the block layer: The molecule got parsed correctly. Creating a block from the
    ///   molecule failed though.
    /// * An error of a lower layer: The parsing of the molecule didn't even work. An error of the
    ///   source is contained.
    Error(Error<S>),

    // General content.

    /// A placeholder to indicate that there is no content.
    ///
    /// For example, an empty document is just this block. A mathematical root without an explicit
    /// root number could contain an empty block.
    Empty,

    /// A text.
    ///
    /// Contains a valid Unicode String.
    Text(String),

    /// A block that gives another block a title.
    ///
    /// Usually, the title teases or summarizes the body.
    Section {
        title: Box<Block<S>>,
        body: Box<Block<S>>,
    },

    /// Displays multiple other blocks one after another without a content break.
    ///
    /// All children should be considered to come immediately after each other. For example, a flow
    /// containing the texts "He" and "llo" should be semantically equivalent to just "Hello".
    Flow(Vec<Block<S>>),

    /// Displays multiple other blocks with a small content break.
    Paragraphs(Vec<Block<S>>),

    /// Displays multiple blocks in a bullet list.
    BulletList(Vec<Block<S>>),

    /// Displays multiple blocks in a numbered list.
    OrderedList(Vec<Block<S>>),
}
use Block::*;

impl<S: Source> Block<S> {
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }
    pub fn simplify(self) -> Block<S> {
        use Block::*;
        match self {
            Error(error) => Error(error),
            Empty => Empty,
            Text(text) => {
                if text.is_empty() {
                    Empty
                } else {
                    Text(text)
                }
            }
            Section { title, body } =>  {
                let title = title.simplify();
                let body = body.simplify();
                if title.is_empty() {
                    body
                } else {
                    Section { title: Box::new(title), body: Box::new(body) }
                }
            }
            Flow(children) => {
                let original_children = children.simplify();
                
                // Merge adjacent texts.
                let mut children = vec![];
                let mut text = "".to_owned();
                for child in original_children {
                    match child {
                        Text(additional_text) => text += &additional_text,
                        other => {
                            if !text.is_empty() {
                                children.push(Text(text));
                                text = "".to_owned();
                            }
                            children.push(other)
                        }
                    }
                }
                if !text.is_empty() {
                    children.push(Text(text));
                }
    
                match children.len() {
                    0 => Empty,
                    1 => children.first().unwrap().clone(),
                    _ => Flow(children)
                }
            },
            Paragraphs(children) => {
                let children = children.simplify();
                match children.len() {
                    0 => Empty,
                    1 => children.first().unwrap().clone(),
                    _ => Paragraphs(children)
                }
            }
            BulletList(items) => BulletList(items.simplify()),
            OrderedList(items) => OrderedList(items.simplify()),
        }
    }
}
trait SimplifyAll<S: Source> {
    fn simplify(self) -> Vec<Block<S>>;
}
impl<S: Source> SimplifyAll<S> for Vec<Block<S>> {
    fn simplify(self) -> Vec<Block<S>> {
        self.into_iter().map(|block| block.simplify()).filter(|block| !block.is_empty()).collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error<S: Source> {
    BlockLayer(BlockError),
    LowerLayer(S::Error),
}

pub mod kinds {
    pub const EMPTY: u64 = 0;
    pub const TEXT: u64 = 1;
    pub const SECTION: u64 = 2;
    pub const FLOW: u64 = 3;
    pub const PARAGRAPHS: u64 = 4;
    pub const BULLET_LIST: u64 = 5;
    pub const ORDERED_LIST: u64 = 6;
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
            Flow(children) => Molecule::block(kinds::FLOW, children.clone().into_molecules()),
            Paragraphs(children) => {
                Molecule::block(kinds::PARAGRAPHS, children.clone().into_molecules())
            }
            BulletList(items) => Molecule::block(kinds::BULLET_LIST, items.clone().into_molecules()),
            OrderedList(items) => Molecule::block(kinds::ORDERED_LIST, items.clone().into_molecules()),
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
            kinds::FLOW => Ok(Flow(
                children.into_blocks(),
            )),
            kinds::PARAGRAPHS => Ok(Paragraphs(
                children.into_blocks(),
            )),
            kinds::BULLET_LIST => Ok(BulletList(children.into_blocks())),
            kinds::ORDERED_LIST => Ok(OrderedList(children.into_blocks())),
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

    pub fn into_pure(self) -> Result<Block<Pure>, S::Error> {
        Ok(match self {
            Error(error) => Error(match error {
                Error::BlockLayer(error) => Error::BlockLayer(error),
                Error::LowerLayer(error) => return Err(error),
            }),
            Empty => Empty,
            Text(text) => Text(text),
            Section { title, body } => Section {
                title: Box::new(title.into_pure()?),
                body: Box::new(body.into_pure()?),
            },
            Flow(children) => Flow(children.into_pure()?),
            Paragraphs(children) => Paragraphs(children.into_pure()?),
            BulletList(items) => BulletList(items.into_pure()?),
            OrderedList(items) => OrderedList(items.into_pure()?),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
trait IntoBlocks<S: Source> {
    fn into_blocks(self) -> Vec<Block<S>>;
}
impl<S: Source> IntoBlocks<S> for Vec<Molecule<S>> {
    fn into_blocks(self) -> Vec<Block<S>> {
        self.into_iter().map(|data| Block::from(&data)).collect()
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
trait VecIntoPure<S: Source> {
    fn into_pure(self) -> Result<Vec<Block<Pure>>, S::Error>;
}
impl<S: Source> VecIntoPure<S> for Vec<Block<S>> {
    fn into_pure(self) -> Result<Vec<Block<Pure>>, S::Error> {
        self.into_iter()
            .map(|item| item.into_pure())
            .collect::<Result<_, _>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for Block<Pure> {
        fn arbitrary(g: &mut Gen) -> Self {
            match u64::arbitrary(g) % 9 {
                // Blocks without children.
                0 | 1 | 2 => Empty,
                3 | 4 | 5 => Text(String::arbitrary(g)),
                // Blocks with two children.
                6 => Section {
                    title: Box::new(Block::arbitrary(g)),
                    body: Box::new(Block::arbitrary(g)),
                },
                // Blocks with a variable number of children.
                7 => Flow(Vec::arbitrary(g)),
                8 => Paragraphs(Vec::arbitrary(g)),
                _ => panic!("Modulo didn't work."),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            match self.clone() {
                Empty => empty_shrinker(),
                Text(text) => Box::new(text.shrink().map(Text)),
                Section { title, body } => {
                    let body_for_closure = body.clone();
                    Box::new(
                        single_shrinker(*title.clone())
                            .chain(single_shrinker(*body.clone()))
                            .chain(title.shrink().map(move |title| Section {
                                title,
                                body: body_for_closure.clone(),
                            }))
                            .chain(body.shrink().map(move |body| Section {
                                title: title.clone(),
                                body,
                            })),
                    )
                }
                Flow(children) => Box::new(children.shrink().map(Flow)),
                Paragraphs(children) => Box::new(children.shrink().map(Paragraphs)),
                Error(error) => panic!(
                    "Error values should never be generated, but we were asked to shrink {:?}.",
                    error
                ),
                BulletList(items) => Box::new(items.shrink().map(BulletList)),
                OrderedList(items) => Box::new(items.shrink().map(OrderedList)),
            }
        }
    }
}
