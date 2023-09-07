use std::{cmp::Ordering, ops::RangeInclusive, rc::Rc};

use crate::{
    append::Append, flipped::FlippedPiratesTree, normal::NormalPiratesTree, pirate::Pirate,
};

pub enum PiratesTreeInner {
    Normal(NormalPiratesTree),
    Flipped(FlippedPiratesTree),
}

impl PiratesTreeInner {
    fn len(&self) -> usize {
        match self {
            Self::Normal(tree) => tree.len(),
            Self::Flipped(tree) => tree.len(),
        }
    }
    fn num_bucaneers(&self) -> usize {
        match self {
            Self::Normal(tree) => tree.num_bucaneers(),
            Self::Flipped(tree) => tree.num_bucaneers(),
        }
    }
    fn unpack(&self) -> Option<(PiratesTree, PiratesTree)> {
        match self {
            Self::Normal(tree) => tree.unpack(),
            Self::Flipped(tree) => tree.unpack(),
        }
    }
    fn flipped(&self) -> PiratesTree {
        match self {
            Self::Normal(tree) => tree.flipped(),
            Self::Flipped(tree) => tree.flipped(),
        }
    }
}

#[derive(Clone)]
pub struct PiratesTree(Rc<PiratesTreeInner>);

impl PiratesTree {
    pub fn from_string(string: &str) -> Self {
        match string.len().cmp(&1) {
            Ordering::Equal => {
                let c = string.chars().next().unwrap();
                PiratesTree::single_pirate(Pirate::from_char(c).unwrap())
            }
            Ordering::Less => panic!("Invalid string"),
            Ordering::Greater => Self::join(
                Self::from_string(&string[..string.len() / 2]),
                Self::from_string(&string[string.len() / 2..]),
            ),
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn num_bucaneers(&self) -> usize {
        self.0.num_bucaneers()
    }

    pub fn join(left_pirates: PiratesTree, right_pirates: PiratesTree) -> Self {
        let size = left_pirates.len() + right_pirates.len();
        let num_bucaneers = left_pirates.num_bucaneers() + right_pirates.num_bucaneers();
        Self(Rc::new(PiratesTreeInner::Normal(NormalPiratesTree::Join {
            left: left_pirates,
            right: right_pirates,
            size,
            num_bucaneers,
        })))
    }

    pub fn replicate(&self, n: usize) -> Self {
        if n == 1 {
            self.clone()
        } else {
            let left = self.replicate(n / 2);
            let mut right = left.clone();
            if n % 2 == 1 {
                right = Self::join(right, self.clone());
            }
            Self::join(left, right)
        }
    }

    pub fn from_sections(sections: Vec<PiratesTree>) -> Self {
        let len = sections.len();
        match len.cmp(&1) {
            Ordering::Equal => sections.into_iter().next().unwrap(),
            Ordering::Greater => {
                let left = sections[..len / 2].to_vec();
                let right = sections[len / 2..].to_vec();
                let left = Self::from_sections(left);
                let right = Self::from_sections(right);
                Self::join(left, right)
            }
            Ordering::Less => panic!("Invalid sections"),
        }
    }

    fn recurse<R: Append>(
        &self,
        range: RangeInclusive<usize>,
        on_none: &impl Fn(&PiratesTree) -> R,
        on_full: &impl Fn(&PiratesTree) -> R,
    ) -> R {
        let start = *range.start();
        let end = *range.end();
        if start == 0 && end == self.len() - 1 {
            return on_full(self);
        }
        let (left, right) = self.unpack().unwrap();
        if end >= self.len() {
            panic!("Weird bounds");
        }
        let left_result = if start < left.len() {
            left.recurse(start..=end.min(left.len() - 1), on_none, on_full)
        } else {
            on_none(&left)
        };
        let right_result = if end >= left.len() {
            right.recurse(
                start.max(left.len()) - left.len()..=end - left.len(),
                on_none,
                on_full,
            )
        } else {
            on_none(&right)
        };
        left_result.append(right_result)
    }

    #[must_use]
    pub fn set(&self, range: RangeInclusive<usize>, pirate_type: Pirate) -> Self {
        self.recurse(
            range,
            &|tree: &PiratesTree| tree.clone(),
            &|tree: &PiratesTree| Self::single_pirate(pirate_type).replicate(tree.len()),
        )
    }

    pub fn toggle(&self, range: RangeInclusive<usize>) -> PiratesTree {
        self.recurse(range, &|tree| tree.clone(), &|tree| tree.flipped())
    }

    pub fn count_bucaneers(&self, range: RangeInclusive<usize>) -> usize {
        self.recurse(range, &|_tree| 0, &|tree| {
            let result = tree.num_bucaneers();

            result
        })
    }

    pub fn flipped(&self) -> Self {
        self.0.flipped()
    }

    fn single_pirate(pirate: Pirate) -> PiratesTree {
        PiratesTree(Rc::new(PiratesTreeInner::Normal(
            NormalPiratesTree::SinglePirate(pirate),
        )))
    }

    fn unpack(&self) -> Option<(PiratesTree, PiratesTree)> {
        self.0.unpack()
    }

    pub(crate) fn new(t: PiratesTreeInner) -> PiratesTree {
        PiratesTree(Rc::new(t))
    }
}
