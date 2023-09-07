use crate::pirates_tree::PiratesTree;

pub trait Append {
    fn append(self, other: Self) -> Self;
}

impl Append for usize {
    fn append(self, other: Self) -> Self {
        self + other
    }
}

impl Append for PiratesTree {
    fn append(self, other: Self) -> Self {
        Self::join(self, other)
    }
}
