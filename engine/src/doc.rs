use std::convert::TryInto;

use crate::blocks::*;
use crate::memory::*;
use crate::source::*;

const MAGIC_BYTES: &[u8] = b"SemDoc";
const VERSION: u16 = 0;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SemDoc<S: Source> {
    pub block: Block<S>,
}
impl<S: Source> SemDoc<S> {
    pub fn new(block: Block<S>) -> Self {
        Self { block }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(MAGIC_BYTES);
        bytes.extend_from_slice(&VERSION.to_be_bytes());
        bytes.extend_from_slice(
            &self
                .block
                .to_molecule()
                .to_atoms()
                .iter()
                .map(|atom| atom.to_bytes())
                .flatten()
                .collect::<Vec<_>>(),
        );
        bytes
    }

    pub fn without_source_errors(self) -> Result<SemDoc<Pure>, S::Error> {
        Ok(SemDoc::<Pure> {
            block: self.block.without_source_errors()?,
        })
    }
}
impl SemDoc<Memory> {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SemDocError> {
        if bytes.len() < 8 {
            return Err(SemDocError::UnexpectedEnd);
        }
        if !bytes.starts_with(MAGIC_BYTES) {
            return Err(SemDocError::MagicBytesInvalid);
        }
        if u16::from_be_bytes(bytes[6..8].try_into().unwrap(/* slice has the right length */))
            != VERSION
        {
            return Err(SemDocError::UnknownVersion);
        }
        let block = Block::from(&MemoryMolecule::from(&bytes[8..]));
        Ok(SemDoc { block })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SemDocError {
    UnexpectedEnd,
    MagicBytesInvalid,
    UnknownVersion,
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::*;

    impl quickcheck::Arbitrary for SemDoc<Pure> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                block: Block::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.block.shrink().map(|block| Self { block }))
        }
    }

    quickcheck! {
        fn prop(doc: SemDoc<Pure>) -> bool {
            let reencoded = match SemDoc::from_bytes(&doc.to_bytes()).map(|doc| doc.without_source_errors()) {
                Ok(Ok(doc)) => doc,
                Ok(Err(_)) => return false,
                Err(_) => return false,
            };
            reencoded == doc
        }
    }
}
