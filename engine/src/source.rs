pub trait Source: Clone + std::fmt::Debug {
    type Error: std::fmt::Debug + Clone;
}
