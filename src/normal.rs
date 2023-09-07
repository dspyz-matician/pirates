use crate::{
    flipped::FlippedPiratesTree,
    pirate::Pirate,
    pirates_tree::{PiratesTree, PiratesTreeInner},
};

#[derive(Clone)]
pub enum NormalPiratesTree {
    SinglePirate(Pirate),
    Join {
        left: PiratesTree,
        right: PiratesTree,
        size: usize,
        num_bucaneers: usize,
    },
}
impl NormalPiratesTree {
    pub fn len(&self) -> usize {
        match self {
            Self::SinglePirate(_) => 1,
            Self::Join { size, .. } => *size,
        }
    }

    pub fn num_bucaneers(&self) -> usize {
        match self {
            Self::SinglePirate(Pirate::Bucaneer) => 1,
            Self::SinglePirate(Pirate::Barbary) => 0,
            &Self::Join { num_bucaneers, .. } => num_bucaneers,
        }
    }

    pub fn unpack(&self) -> Option<(PiratesTree, PiratesTree)> {
        match self {
            Self::SinglePirate(_) => None,
            Self::Join { left, right, .. } => Some((left.clone(), right.clone())),
        }
    }

    pub fn flipped(&self) -> PiratesTree {
        PiratesTree::new(PiratesTreeInner::Flipped(FlippedPiratesTree::flip(
            self.clone(),
        )))
    }
}
