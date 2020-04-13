use crate::utils::SourceRange;
use std::ops::Deref;

/// A node in the abstract syntax tree. Implementing this with a struct and an
/// enum rather than just an enum allows for storing additional [Metadata]
/// (like, the position in the original source) along with the actual [Element]
/// inside each [Node].
#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub element: Element,
    pub metadata: Metadata,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.element.fmt(f)
    }
}

/// The actual data of each [Element].
#[derive(Debug, Eq, PartialEq)]
pub enum Element {
    Text(String),
    Block { name: String, bodies: Vec<Body> },
}
pub type Body = Vec<Node>;

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Element::Text(text) => write!(f, "{}", text),
            Element::Block { name, bodies } => write!(
                f,
                "[{}]{}",
                name,
                bodies
                    .iter()
                    .map(|body| body
                        .iter()
                        .map(|node| format!("{}", node))
                        .collect::<Vec<String>>()
                        .join(""))
                    .map(|body_content| format!("{{{}}}", body_content))
                    .collect::<Vec<String>>()
                    .join("")
            ),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Metadata {
    pub position: SourceRange,
}

impl Deref for Node {
    type Target = Element;
    fn deref(&self) -> &Element {
        &self.element
    }
}

/// Pattern that applies the given pattern [p] to the [element] of a [Node],
/// making the matching independent from the [Node]'s [data].
#[macro_export]
macro_rules! AnyData {
    ($p:pat) => {
        crate::ssss::tree::Node { element: $p, .. }
    };
}
