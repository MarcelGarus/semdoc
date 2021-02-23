use colored::{Color, Colorize};
use semdoc_engine::atoms::*;
use semdoc_engine::utils::*;
use std::cmp::min;
use std::convert::TryInto;

use crate::utils::*;

enum WordInfo {
    Header { version: u16 },
    Block { id: Id, kind: u64, num_children: u8 },
    Bytes { id: Id, length: u64 },
    FewBytes { id: Id, length: u8 },
    BytesContinuation { num_relevant: u8, is_last: bool },
}

fn info_for_bytes(bytes: &[u8]) -> Vec<WordInfo> {
    let mut info = vec![WordInfo::Header {
        version: u16::from_be_bytes(bytes[6..8].try_into().unwrap()),
    }];
    // TODO(marcelgarus): Handle too large version.

    let add_byte_continuations = |length: usize, info: &mut Vec<WordInfo>| {
        let num_payload_words = length.round_up_to_multiple_of(8) / 8;
        for i in 0..num_payload_words {
            info.push(if i == num_payload_words - 1 {
                let mut num_relevant = (length % 8) as u8;
                if num_relevant == 0 {
                    num_relevant = 8;
                }
                WordInfo::BytesContinuation {
                    num_relevant,
                    is_last: true,
                }
            } else {
                WordInfo::BytesContinuation {
                    num_relevant: 8,
                    is_last: false,
                }
            });
        }
    };

    let mut cursor = 8;
    let mut id = 0;
    while cursor < bytes.len() {
        let atom = Atom::from_bytes(&bytes[cursor..]).unwrap();
        cursor += 8 * atom.length_in_words();
        match atom {
            Atom::Block { kind, num_children } => info.push(WordInfo::Block {
                id,
                kind,
                num_children,
            }),
            Atom::Reference(_offset) => todo!("Handle references."),
            Atom::Bytes(bytes) => {
                info.push(WordInfo::Bytes {
                    id,
                    length: bytes.len() as u64,
                });
                add_byte_continuations(bytes.len(), &mut info);
            }
            Atom::FewBytes(bytes) => {
                info.push(WordInfo::FewBytes {
                    id,
                    length: bytes.len() as u8,
                });
                add_byte_continuations(
                    // Up to 6 bytes are already stored right here.
                    if bytes.len() < 6 { 0 } else { bytes.len() - 6 },
                    &mut info,
                );
            }
        }
        id += 1;
    }
    info
}

impl WordInfo {
    fn to_byte_styles(&self) -> [Color; 8] {
        use colors::*;
        match self {
            WordInfo::Header { .. } => [MAGIC, MAGIC, MAGIC, MAGIC, MAGIC, MAGIC, VERSION, VERSION],
            WordInfo::Block { .. } => [ATOM_KIND, NUM_CHILDREN, KIND, KIND, KIND, KIND, KIND, KIND],
            WordInfo::Bytes { .. } => [
                ATOM_KIND, LENGTH, LENGTH, LENGTH, LENGTH, LENGTH, LENGTH, LENGTH,
            ],
            WordInfo::FewBytes { length: len, .. } => {
                let mut colors = [PADDING; 8];
                for i in 0..min(2 + *len as usize, 8) {
                    colors[i] = PAYLOAD;
                }
                colors[0] = ATOM_KIND;
                colors[1] = LENGTH;
                colors
            }
            WordInfo::BytesContinuation { num_relevant, .. } => {
                let mut colors = [PADDING; 8];
                for i in 0..(*num_relevant as usize) {
                    colors[i] = PAYLOAD;
                }
                colors
            }
        }
    }
}

pub fn inspect_bytes(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let info = info_for_bytes(&bytes);
    let mut children_left = vec![]; // How many children are left in each indentation level.

    println!(
        "{:4}  {:23}  {:8}  {}",
        "Word".bold(),
        "Bytes".bold(),
        "ASCII".bold(),
        "Info".bold(),
    );
    for (index, word) in bytes.chunks(8).enumerate() {
        let info = &info[index];

        println!(
            "{}  {}  {}  {}{}",
            format_word_index(index),
            format_bytes(word, info),
            format_bytes_ascii(word, info),
            format_tree(&children_left, info.starts_atom()),
            format_info(info),
        );

        if info.starts_atom() {
            let num_layers = children_left.len();
            if num_layers > 0 && children_left[num_layers - 1] > 0 {
                children_left[num_layers - 1] -= 1;
            }
        }
        if let WordInfo::Block { num_children, .. } = info {
            children_left.push(*num_children as usize);
        }
        if info.ends_atom() {
            while matches!(children_left.last(), Some(left) if *left == 0) {
                children_left.pop();
            }
        }
    }
}
impl WordInfo {
    fn starts_atom(&self) -> bool {
        matches!(
            self,
            WordInfo::Block { .. } | WordInfo::Bytes { .. } | WordInfo::FewBytes { .. }
        )
    }
    fn ends_atom(&self) -> bool {
        match self {
            WordInfo::BytesContinuation { is_last, .. } if *is_last => true,
            WordInfo::FewBytes { length, .. } if *length <= 6 => true,
            _ => false,
        }
    }
}

fn format_word_index(index: usize) -> String {
    format!("{:4}", index)
}
fn format_bytes(word: &[u8], info: &WordInfo) -> String {
    let styles = info.to_byte_styles();
    word.iter()
        .enumerate()
        .map(|(i, byte)| format!("{}", format!("{:02x}", byte).color(styles[i])))
        .collect::<Vec<_>>()
        .join(" ")
}
fn format_bytes_ascii(word: &[u8], info: &WordInfo) -> String {
    let styles = info.to_byte_styles();
    word.iter()
        .enumerate()
        .map(|(i, byte)| byte.ascii_or_dot().to_string().color(styles[i]).to_string())
        .collect::<Vec<_>>()
        .join("")
}
fn format_info(info: &WordInfo) -> String {
    match info {
        WordInfo::Header { version } => {
            format!(
                "Header with {}{}",
                "magic bytes, ".color(colors::MAGIC),
                format!("SemDoc version {}", version).color(colors::VERSION),
            )
        }
        WordInfo::Block {
            id,
            kind,
            num_children,
        } => format_atom_block_header(*id, *kind, *num_children),
        WordInfo::Bytes { id, length } => format_atom_bytes_header(*id, *length as usize),
        WordInfo::FewBytes { id, length } => {
            format_atom_few_bytes_header(*id, *length as usize, true)
        }
        WordInfo::BytesContinuation { num_relevant, .. } => format_payload_label(*num_relevant < 8),
    }
}
