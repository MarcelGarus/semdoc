use super::engine::atoms::Atom;

#[derive(Debug)]
pub enum Block {
    // Special.
    Unknown { kind: u64, children: Vec<Block> },
    Empty,

    // Meta.
    Created,

    // Content.
    Text(String),
    Section { title: Box<Block>, body: Box<Block> },
    DenseSequence(Vec<Block>),
    SplitSequence(Vec<Block>),

    // Collaboration.
    Comment,
}

pub trait SerializeToAtom {
    fn serialize(&self) -> Atom;
}
impl SerializeToAtom for Block {
    fn serialize(&self) -> Atom {
        use Block::*;

        match self {
            Unknown { kind, children } => Atom::Block {
                kind: *kind,
                children: children
                    .into_iter()
                    .map(|child| child.serialize())
                    .collect(),
            },
            Empty => Atom::Block {
                kind: 0,
                children: vec![],
            },
            Created => Atom::Block {
                kind: 1,
                children: vec![],
            },
            Section { title, body } => Atom::Block {
                kind: 2,
                children: vec![title.serialize(), body.serialize()],
            },
            DenseSequence(children) => Atom::Block {
                kind: 3,
                children: children
                    .into_iter()
                    .map(|child| child.serialize())
                    .collect(),
            },
            SplitSequence(children) => Atom::Block {
                kind: 4,
                children: children
                    .into_iter()
                    .map(|child| child.serialize())
                    .collect(),
            },
            Text(text) => Atom::Block {
                kind: 5,
                children: vec![Atom::Bytes(text.as_bytes())],
            },
            Comment => Atom::Block {
                kind: 6,
                children: vec![],
            },
        }
    }
}

pub trait DeserializeToBlock {
    fn deserialize(&self) -> Result<Block, ()>;
}
impl<'a> DeserializeToBlock for Atom<'a> {
    fn deserialize(&self) -> Result<Block, ()> {
        use Block::*;

        Ok(match self {
            Atom::Bytes(_) => return Err(()),
            Atom::Block { kind, children } => match kind {
                0 => Empty,
                1 => Created,
                2 => Section {
                    title: Box::new(children.get(0).unwrap().deserialize().unwrap()),
                    body: Box::new(children.get(1).unwrap().deserialize().unwrap()),
                },
                3 => DenseSequence(
                    children
                        .into_iter()
                        .map(|child| child.deserialize().unwrap())
                        .collect(),
                ),
                4 => SplitSequence(
                    children
                        .into_iter()
                        .map(|child| child.deserialize().unwrap())
                        .collect(),
                ),
                5 => Text(match *children.first().unwrap() {
                    Atom::Bytes(bytes) => String::from_utf8(bytes.to_vec()).unwrap(),
                    _ => return Err(()),
                }),
                6 => Comment,
                _ => Unknown {
                    kind: *kind,
                    children: children
                        .into_iter()
                        .map(|child| child.deserialize().unwrap())
                        .collect(),
                },
            },
        })
    }
}
