use super::utils::*;

pub type AtomKind = u64;

// TODO(marcelgarus): Add atom for packed bytes.
#[derive(Clone, Debug)]
pub enum Atom {
    Block { kind: AtomKind, num_children: u8 },
    Reference(u64),
    Bytes(Vec<u8>),
    FewBytes(Vec<u8>),
}

#[derive(Debug)]
pub enum AtomError {
    UnexpectedEnd,
    UnknownType(u8),
    AlignmentNotZero,
}

impl Atom {
    pub fn length_in_words(&self) -> usize {
        use Atom::*;

        match self {
            Block { .. } => 1,
            Reference(_) => 1,
            Bytes(bytes) => 1 + bytes.len().round_up_to_multiple_of(8) / 8,
            FewBytes(bytes) => {
                1 + (if bytes.len() < 6 { 0 } else { bytes.len() - 6 }).round_up_to_multiple_of(8)
                    / 8
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        use Atom::*;

        match self {
            Block { num_children, kind } => {
                assert!(
                    *kind <= MAX_VALUE_USING_6_BYTES,
                    "The kind of an Atom::Block is too big. The maximum supported kind is {}.",
                    MAX_VALUE_USING_6_BYTES
                );
                let mut bytes = vec![0];
                bytes.push(*num_children);
                bytes.extend_from_slice(&kind.to_be_bytes()[2..]);
                bytes
            }
            Reference(offset) => {
                assert!(*offset <= MAX_VALUE_USING_7_BYTES, "The offset of an Atom::Reference is too big. The maximum supported offset is {}.", MAX_VALUE_USING_7_BYTES);
                let mut bytes = vec![1];
                bytes.extend_from_slice(&offset.to_be_bytes()[1..]);
                bytes
            }
            Bytes(payload_bytes) => {
                assert!(payload_bytes.len() <= MAX_VALUE_USING_7_BYTES as usize, "The bytes saved in an Atom::Bytes are too long. The maximum supported length is {}.", MAX_VALUE_USING_7_BYTES);
                let mut bytes = vec![2];
                bytes.extend_from_slice(&(payload_bytes.len() as u64).to_be_bytes()[1..]);
                bytes.extend_from_slice(&payload_bytes);
                bytes.align();
                bytes
            }
            FewBytes(payload_bytes) => {
                assert!(payload_bytes.len() <= u8::MAX as usize, "The bytes saved in an Atom::FewBytes are too long. The maximum supported length is {}.", u8::MAX);
                let mut bytes = vec![3];
                bytes.push(payload_bytes.len() as u8);
                bytes.extend_from_slice(&payload_bytes);
                bytes.align();
                bytes
            }
        }
    }

    pub fn try_from(bytes: &[u8]) -> Result<Atom, AtomError> {
        use Atom::*;
        if bytes.len() < 8 {
            return Err(AtomError::UnexpectedEnd);
        }
        Ok(match bytes.first().ok_or(AtomError::UnexpectedEnd)? {
            0 => Block {
                kind: u64::clone_from_slice(&bytes[2..8]),
                num_children: *bytes.get(1).ok_or(AtomError::UnexpectedEnd)?,
            },
            1 => Reference(u64::clone_from_slice(&bytes[1..8])),
            2 => {
                let length = (u64::clone_from_slice(&bytes[1..8])) as usize;
                if bytes.len() < 8 + length {
                    return Err(AtomError::UnexpectedEnd);
                }
                let payload_bytes = &bytes[8..(8 + length)];
                if bytes[(8 + length)..(8 + length).round_up_to_multiple_of(8)]
                    .iter()
                    .any(|byte| *byte != 0)
                {
                    return Err(AtomError::AlignmentNotZero);
                }
                Bytes(payload_bytes.to_vec())
            }
            3 => {
                let length = bytes[1] as usize;
                if bytes.len() < 2 + length {
                    return Err(AtomError::UnexpectedEnd);
                }
                let payload_bytes = &bytes[2..(2 + length)];
                // TODO(marcelgarus): Check alignment bytes.
                FewBytes(payload_bytes.to_vec())
            }
            type_ => return Err(AtomError::UnknownType(*type_)),
        })
    }
}

const MAX_VALUE_USING_6_BYTES: u64 = 281474976710656 - 1;
const MAX_VALUE_USING_7_BYTES: u64 = 72057594037927936 - 1;
