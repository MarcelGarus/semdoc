use colored::{Color, Colorize};
use semdoc::{Atom, AtomError};
use std::cmp::min;
use std::convert::TryInto;

use super::utils::*;

mod colors {
    use colored::Color;

    pub const ERROR: Color = Color::Red;
    pub const MAGIC: Color = Color::BrightMagenta;
    pub const VERSION: Color = Color::Yellow;
    pub const ATOM_KIND: Color = Color::Yellow;
    pub const NUM_CHILDREN: Color = Color::BrightRed;
    pub const KIND: Color = Color::Green;
    pub const LENGTH: Color = Color::BrightRed;
    pub const PAYLOAD: Color = Color::BrightCyan;
    pub const PADDING: Color = Color::Blue;
}

enum WordInfo {
    Error { error: AtomError },
    ErrorContinuation,
    Header { version: u16 },
    Block { kind: u64 },
    BlockContinuation { num_children: u64 },
    SmallBlock { kind: u64, num_children: u8 },
    Bytes { length: u64 },
    FewBytes { length: u8 },
    BytesContinuation { num_relevant: u8 },
}

pub fn inspect_bytes(file: &str) {
    let bytes = std::fs::read(file).expect("File not found.");
    let info = info_for_bytes(&bytes);

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
            "{:4}  {}  {}  {}",
            index,
            format_bytes_hex(word, info),
            format_bytes_ascii(word, info),
            format_info(info),
        );
    }
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
                WordInfo::BytesContinuation { num_relevant }
            } else {
                WordInfo::BytesContinuation { num_relevant: 8 }
            });
        }
    };

    let mut cursor = 8;
    while cursor < bytes.len() {
        let atom = match Atom::try_from(&bytes[cursor..]) {
            Ok(atom) => atom,
            Err(error) => {
                info.push(WordInfo::Error { error });
                cursor += 8;
                while cursor < bytes.len() {
                    info.push(WordInfo::ErrorContinuation);
                    cursor += 8;
                }
                break;
            }
        };
        cursor += atom.length_in_bytes();
        match atom {
            Atom::Block { kind, num_children } => {
                info.push(WordInfo::Block { kind });
                info.push(WordInfo::BlockContinuation { num_children });
            }
            Atom::SmallBlock { kind, num_children } => {
                info.push(WordInfo::SmallBlock { kind, num_children })
            }
            Atom::Reference(_offset) => todo!("Handle references."),
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
                add_byte_continuations(
                    // Up to 6 bytes are already stored right here.
                    if bytes.len() < 6 { 0 } else { bytes.len() - 6 },
                    &mut info,
                );
            }
        }
    }
    info
}

impl WordInfo {
    fn to_byte_styles(&self) -> [Color; 8] {
        use colors::*;
        match self {
            WordInfo::Error { .. } | WordInfo::ErrorContinuation => [ERROR; 8],
            WordInfo::Header { .. } => [MAGIC, MAGIC, MAGIC, MAGIC, MAGIC, MAGIC, VERSION, VERSION],
            WordInfo::Block { .. } => [ATOM_KIND, PADDING, KIND, KIND, KIND, KIND, KIND, KIND],
            WordInfo::BlockContinuation { .. } => [NUM_CHILDREN; 8],
            WordInfo::SmallBlock { .. } => {
                [ATOM_KIND, NUM_CHILDREN, KIND, KIND, KIND, KIND, KIND, KIND]
            }
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

fn format_bytes_hex(word: &[u8], info: &WordInfo) -> String {
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
        .map(|(i, byte)| {
            byte.ascii_or_none()
                .unwrap_or('.')
                .to_string()
                .color(styles[i])
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("")
}

fn format_info(info: &WordInfo) -> String {
    match info {
        WordInfo::Error { error } => format!("Error: {:?}", error).red().to_string(),
        WordInfo::ErrorContinuation => format!(""),
        WordInfo::Header { version } => {
            format!(
                "Header with {}{}",
                "magic bytes, ".color(colors::MAGIC),
                format!("SemDoc version {}", version).color(colors::VERSION),
            )
        }
        WordInfo::Block { kind } => format!(
            "{}{}{}",
            format_atom_kind("Block"),
            "padding, ".color(colors::PADDING),
            format!("kind {} ({}), ", kind, kind_to_name(*kind)).color(colors::KIND),
        ),
        WordInfo::BlockContinuation { num_children } => format!(
            "{} {}",
            num_children,
            singular_or_plural(*num_children as usize, "child", "children")
        )
        .color(colors::NUM_CHILDREN)
        .to_string(),
        WordInfo::SmallBlock { kind, num_children } => format!(
            "{}{}{}",
            format_atom_kind("SmallBlock"),
            format!("kind {} ({}), ", kind, kind_to_name(*kind)).color(colors::KIND),
            format!(
                "{} {}",
                num_children,
                singular_or_plural(*num_children as usize, "child", "children")
            )
            .color(colors::NUM_CHILDREN),
        ),
        WordInfo::Bytes { length } => format!(
            "{}{}",
            format_atom_kind("Bytes"),
            format_n_bytes_long(*length as usize, false),
        ),
        WordInfo::FewBytes { length } => {
            format!(
                "{}{}",
                format_atom_kind("FewBytes"),
                format_n_bytes_long(*length as usize, false),
            )
        }
        WordInfo::BytesContinuation { num_relevant, .. } => format!(
            "{}{}",
            "Payload".color(colors::PAYLOAD),
            if *num_relevant < 8 {
                " + padding".color(colors::PADDING).to_string()
            } else {
                "".to_string()
            },
        ),
    }
}

fn format_atom_kind(atom_type: &str) -> String {
    format!("{}, ", atom_type)
        .color(colors::ATOM_KIND)
        .bold()
        .to_string()
}

fn format_n_bytes_long(num_bytes: usize, trailing_comma: bool) -> String {
    format!(
        "{} {} long{}",
        num_bytes,
        singular_or_plural(num_bytes, "byte", "bytes"),
        match trailing_comma {
            true => ", ",
            false => "",
        }
    )
    .color(colors::LENGTH)
    .to_string()
}
