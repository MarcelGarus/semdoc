pub trait Align {
    fn align(&mut self);
}
impl Align for Vec<u8> {
    fn align(&mut self) {
        let length = self.len();
        let filling_amount = length.round_up_to_multiple_of(8) - length;
        for _ in 0..filling_amount {
            self.push(0);
        }
    }
}

pub trait CloneFromSlice {
    fn clone_from_slice(bytes: &[u8]) -> Self;
}
impl CloneFromSlice for u64 {
    fn clone_from_slice(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 8);
        let mut tmp = [0u8; 8];
        tmp.clone_from_slice(&bytes);
        u64::from_be_bytes(tmp)
    }
}

pub trait RoundUpToMultipleOf {
    fn round_up_to_multiple_of(&self, number: Self) -> Self;
}
impl RoundUpToMultipleOf for usize {
    fn round_up_to_multiple_of(&self, number: Self) -> Self {
        self + (number - self % number) % number
    }
}
