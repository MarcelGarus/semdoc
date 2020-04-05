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
