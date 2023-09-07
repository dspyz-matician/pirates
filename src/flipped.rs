use crate::{
    normal::NormalPiratesTree,
    pirates_tree::{PiratesTree, PiratesTreeInner},
};

pub struct FlippedPiratesTree(NormalPiratesTree);

impl FlippedPiratesTree {
    pub fn flip(normal: NormalPiratesTree) -> FlippedPiratesTree {
        FlippedPiratesTree(normal)
    }
}

impl PiratesTreeInner for FlippedPiratesTree {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn num_bucaneers(&self) -> usize {
        self.0.len() - self.0.num_bucaneers()
    }

    fn unpack(&self) -> Option<(PiratesTree, PiratesTree)> {
        let (left, right) = self.0.unpack()?;
        Some((left.flipped(), right.flipped()))
    }

    fn flipped(&self) -> PiratesTree {
        PiratesTree::new(self.0.clone())
    }
}
