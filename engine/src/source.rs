pub trait Source: Clone + std::fmt::Debug {
    type Error: std::fmt::Debug + Clone + Eq + PartialEq;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pure();
impl Source for Pure {
    type Error = ();
}
