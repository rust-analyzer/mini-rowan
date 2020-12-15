use std::ops::{AddAssign, Sub};

#[derive(Copy, Clone)]
pub enum Delta<T> {
    Add(T),
    Sub(T),
}

impl<T: Ord + Sub<Output = T>> Delta<T> {
    pub fn new(old: T, new: T) -> Delta<T> {
        if new > old {
            Delta::Sub(new - old)
        } else {
            Delta::Add(old - new)
        }
    }
}

// This won't be coherent :-(
// impl<T: AddAssign + SubAssign> AddAssign<Delta<T>> for T
impl AddAssign<Delta<usize>> for usize {
    fn add_assign(&mut self, rhs: Delta<usize>) {
        match rhs {
            Delta::Add(amt) => *self += amt,
            Delta::Sub(amt) => *self -= amt,
        }
    }
}
