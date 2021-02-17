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

pub trait ToAtom {
    fn to_atom(&self) -> Atom;
}
impl ToAtom for Block {
    fn to_atom(&self) -> Atom {
        use Block::*;

        match self {
            Unknown { kind, children } => Atom::Block {
                kind: *kind,
                children: children.iter().map(|child| child.to_atom()).collect(),
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
                children: vec![title.to_atom(), body.to_atom()],
            },
            DenseSequence(children) => Atom::Block {
                kind: 3,
                children: children.iter().map(|child| child.to_atom()).collect(),
            },
            SplitSequence(children) => Atom::Block {
                kind: 4,
                children: children.iter().map(|child| child.to_atom()).collect(),
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

pub trait ToBlock {
    fn to_block(&self) -> Result<Block, ()>;
}
impl<'a> ToBlock for Atom<'a> {
    fn to_block(&self) -> Result<Block, ()> {
        use Block::*;

        Ok(match self {
            Atom::Bytes(_) => return Err(()),
            Atom::Block { kind, children } => match kind {
                0 => Empty,
                1 => Created,
                2 => Section {
                    title: Box::new(children.get(0).unwrap().to_block().unwrap()),
                    body: Box::new(children.get(1).unwrap().to_block().unwrap()),
                },
                3 => DenseSequence(
                    children
                        .iter()
                        .map(|child| child.to_block().unwrap())
                        .collect(),
                ),
                4 => SplitSequence(
                    children
                        .iter()
                        .map(|child| child.to_block().unwrap())
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
                        .iter()
                        .map(|child| child.to_block().unwrap())
                        .collect(),
                },
            },
        })
    }
}
