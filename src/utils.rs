use std::ops::Deref;
use std::ops::Range;

pub trait Single<T> {
    fn single(&self) -> Option<&T>;
}

impl<T> Single<T> for [T] {
    fn single(&self) -> Option<&T> {
        if self.len() == 1 {
            Some(&self[0])
        } else {
            None
        }
    }
}

/// Returns a lambda that returns [true] for elements that match the given
/// pattern and [false] for the rest.
#[macro_export]
macro_rules! matcher {
    ($p:pat) => {
        |value| matches!(value, $p)
    };
}

/// Position in the source file in bytes.
pub type SourceRange = Range<usize>;

/// A higher-level abstraction of the source.
#[derive(Debug, Eq, PartialEq)]
pub struct Positioned<T> {
    pub data: T,
    pub position: SourceRange,
}

impl<T> Deref for Positioned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}

/// Pattern that applies the given pattern [p] to the [data] of a [Positioned],
/// making the matching independent from the [Positioned]'s [position].
#[macro_export]
macro_rules! Anywhere {
    ($p:pat) => {
        crate::utils::Positioned { data: $p, .. }
    };
}
