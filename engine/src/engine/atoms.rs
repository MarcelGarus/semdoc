use std::convert::TryFrom;

pub type AtomKind = Vec<u8>;

#[derive(Debug)]
pub enum Atom {
    Block { kind: AtomKind, children: Vec<Atom> },
    Bytes(Vec<u8>),
}

impl Atom {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Atom::Block { kind, children } => {
                let mut bytes = vec![0];
                bytes.push(u8::try_from(children.len()).unwrap());
                bytes.extend_from_slice(&kind);
                bytes.push(0);
                bytes.align();
                for child in children {
                    let child_bytes = child.to_bytes();
                    bytes.extend_from_slice(&child_bytes);
                }
                bytes
            }
            Atom::Bytes(payload_bytes) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(&[0u8; 6]);
                bytes.push(u8::try_from(payload_bytes.len()).unwrap());
                bytes.extend_from_slice(&payload_bytes);
                bytes.align();
                bytes
            }
        }
    }

    pub fn from<T: Iterator<Item = u8>>(bytes: &mut T) -> Self {
        match bytes.next().unwrap() {
            0 => {
                let mut counter = 1; // Counter used for alignment.

                let num_children = bytes.next().unwrap();
                counter += 1;

                let mut kind = vec![];
                loop {
                    let byte = bytes.next().unwrap();
                    counter += 1;
                    if byte == 0 {
                        break;
                    }
                    kind.push(byte);
                }

                let alignment_fill = 8 - counter % 8;
                for _ in 0..alignment_fill {
                    let byte = bytes.next().unwrap();
                    if byte != 0 {
                        panic!("Byte should be zero, but is {}", byte);
                    }
                }

                let children = (0..num_children).map(|_| Atom::from(bytes)).collect();

                Atom::Block { kind, children }
            }
            1 => {
                let mut length: u64 = 0;
                for _ in 0..7 {
                    length <<= 8;
                    let next_byte: u64 = bytes.next().unwrap().into();
                    length |= next_byte;
                }

                let payload_bytes = (0..length).map(|_| bytes.next().unwrap()).collect();

                let alignment_fill = 8 - length % 8;
                for _ in 0..alignment_fill {
                    let byte = bytes.next().unwrap();
                    if byte != 0 {
                        panic!("Byte should be zero, but is {}.", byte);
                    }
                }

                Atom::Bytes(payload_bytes)
            }
            kind => panic!("Unknown kind {}.", kind),
        }
    }
}

trait Align {
    fn align(&mut self);
}
impl Align for Vec<u8> {
    fn align(&mut self) {
        let length = self.len();
        let filling_amount = 8 - length % 8;
        for _ in 0..filling_amount {
            self.push(0);
        }
    }
}
