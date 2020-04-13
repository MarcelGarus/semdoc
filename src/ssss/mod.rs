// This module implements functionality for the ssss layer, namely parsing and
// formatting of ssss files.

#[macro_use]
pub mod tree;

mod error;
mod format;
mod parse;
mod scan;

pub use format::{format, FormatConfig};
pub use parse::parse;
pub use tree::*;
