use colored::{Color, Colorize};
use semdoc_engine::atoms::*;
use semdoc_engine::utils::*;
use std::cmp::min;
use std::convert::TryInto;

enum WordInfo {
    Header { version: u16 },
    Block { kind: u64, num_children: u8 },
    Bytes { length: u64 },
    FewBytes { length: u8 },
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
                WordInfo::BytesContinuation {
                    num_relevant: (length % 8) as u8,
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
    while cursor < bytes.len() {
        let atom = Atom::from_bytes(&bytes[cursor..]).unwrap();
        cursor += 8 * atom.length_in_words();
        match atom {
            Atom::Block { kind, num_children } => info.push(WordInfo::Block { kind, num_children }),
            Atom::Reference(offset) => panic!("Handle references."),
            Atom::Bytes(bytes) => {
                info.push(WordInfo::Bytes {
                    length: bytes.len() as u64,
                });
                add_byte_continuations(bytes.len(), &mut info);
            }
            Atom::FewBytes(bytes) => {
                info.push(WordInfo::FewBytes {
                    length: bytes.len() as u8,
                });
                add_byte_continuations(bytes.len() - 6, &mut info); // 6 bytes are already stored right here.
            }
        };
    }
    info
}

mod colors {
    use super::Color;

    pub const MAGIC: Color = Color::BrightMagenta;
    pub const VERSION: Color = Color::Yellow;
    pub const ATOM_KIND: Color = Color::Yellow;
    pub const NUM_CHILDREN: Color = Color::BrightRed;
    pub const KIND: Color = Color::Green;
    pub const LENGTH: Color = Color::BrightRed;
    pub const PAYLOAD: Color = Color::BrightCyan;
    pub const PADDING: Color = Color::Blue;
}

impl WordInfo {
    fn to_byte_styles(&self) -> [Color; 8] {
        use colors::*;
        let w = Color::White;
        match self {
            WordInfo::Header { .. } => [MAGIC, MAGIC, MAGIC, MAGIC, MAGIC, MAGIC, VERSION, VERSION],
            WordInfo::Block { .. } => [ATOM_KIND, NUM_CHILDREN, KIND, KIND, KIND, KIND, KIND, KIND],
            WordInfo::Bytes { .. } => [
                ATOM_KIND, LENGTH, LENGTH, LENGTH, LENGTH, LENGTH, LENGTH, LENGTH,
            ],
            WordInfo::FewBytes { length: len } => {
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
            _ => [w, w, w, w, w, w, w, w],
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
            stringify_word_index(index),
            stringify_bytes(word, info),
            stringify_bytes_ascii(word, info),
            stringify_tree(&children_left, info),
            stringify_info(info),
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
            WordInfo::FewBytes { length } if *length <= 6 => true,
            _ => false,
        }
    }
}

fn stringify_word_index(index: usize) -> String {
    format!("{:4}", index)
}
fn stringify_bytes(word: &[u8], info: &WordInfo) -> String {
    let styles = info.to_byte_styles();
    word.iter()
        .enumerate()
        .map(|(i, byte)| format!("{}", format!("{:02x}", byte).color(styles[i])))
        .collect::<Vec<_>>()
        .join(" ")
}
fn stringify_bytes_ascii(word: &[u8], info: &WordInfo) -> String {
    let styles = info.to_byte_styles();
    word.iter()
        .enumerate()
        .map(|(i, byte)| {
            (if (32..=126).contains(byte) {
                format!("{}", *byte as char)
            } else {
                ".".to_string()
            })
            .color(styles[i])
            .to_string()
        })
        .collect::<Vec<_>>()
        .join("")
}
fn stringify_tree(children_left: &[usize], info: &WordInfo) -> String {
    children_left
        .iter()
        .enumerate()
        .map(|(index, left)| (index == children_left.len() - 1, left))
        .map(|(is_last_layer, left)| match left {
            0 => "  ",
            _ if !is_last_layer => "│ ",
            1 if info.starts_atom() => "└─",
            _ if info.starts_atom() => "├─",
            _ => "│ ",
        })
        .collect::<Vec<_>>()
        .join("")
}
fn stringify_info(info: &WordInfo) -> String {
    match info {
        WordInfo::Header { version } => {
            format!(
                "Header with {}{}",
                "magic bytes, ".color(colors::MAGIC),
                format!("SemDoc version {}", version).color(colors::VERSION),
            )
        }
        WordInfo::Block { kind, num_children } => format!(
            "{}{}{}",
            "Block, ".color(colors::ATOM_KIND).bold(),
            format!("kind {}, ", kind).color(colors::KIND),
            format!(
                "{} {}",
                num_children,
                match num_children {
                    1 => "child",
                    _ => "children",
                }
            )
            .color(colors::NUM_CHILDREN),
        ),
        WordInfo::Bytes { length } => format!(
            "{}{}",
            "Bytes, ".color(colors::ATOM_KIND).bold(),
            format!("{} bytes long", length).color(colors::LENGTH),
        ),
        WordInfo::FewBytes { length } => format!(
            "{}{}",
            "FewBytes, ".color(colors::ATOM_KIND).bold(),
            format!("{} bytes long", length).color(colors::LENGTH),
        ),
        WordInfo::BytesContinuation { num_relevant, .. } => format!(
            "{}{}",
            "Payload".color(colors::PAYLOAD),
            match num_relevant {
                8 => "".to_owned(),
                _ => " + padding".color(colors::PADDING).to_string(),
            }
        ),
    }
}
