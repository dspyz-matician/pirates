use std::{
    cmp::Ordering,
    convert::Infallible,
    io::{stdin, Read},
    ops::RangeInclusive,
    rc::Rc,
    str::FromStr,
};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let mut tokens = s.split_whitespace();
    let num_cases = tokens.next().unwrap().parse::<usize>().unwrap();
    for case in 0..num_cases {
        println!("Case {case}:");
        let num_strings = tokens.next().unwrap().parse::<usize>().unwrap();
        let mut sections = Vec::with_capacity(num_strings);
        for _i in 0..num_strings {
            let repetitions = tokens.next().unwrap().parse::<usize>().unwrap();
            let string = tokens.next().unwrap();
            let tree = PiratesTree::from_string(string);
            sections.push(tree.replicate(repetitions));
        }
        let mut tree = PiratesTree::from_sections(sections);
        eprintln!("{}", tree.to_string());

        let num_instructions = tokens.next().unwrap().parse::<usize>().unwrap();
        let mut queries_encountered = 0;
        for _i in 0..num_instructions {
            let instruction_type = tokens
                .next()
                .unwrap()
                .parse::<InstructionType>()
                .unwrap_or_else(|err| match err {});
            let start_index = tokens.next().unwrap().parse::<usize>().unwrap();
            let end_index = tokens.next().unwrap().parse::<usize>().unwrap();
            match instruction_type {
                InstructionType::MakeBarbary => {
                    tree = tree.set(start_index..=end_index, Pirate::Barbary)
                }
                InstructionType::MakeBucaneer => {
                    tree = tree.set(start_index..=end_index, Pirate::Bucaneer)
                }
                InstructionType::Toggle => tree = tree.toggle(start_index..=end_index),
                InstructionType::GodsQuery => {
                    queries_encountered += 1;
                    println!(
                        "Q{}: {}",
                        queries_encountered,
                        tree.count_bucaneers(start_index..=end_index)
                    );
                }
            }
            eprintln!("{}", tree.to_string());
        }
    }
}

enum InstructionType {
    MakeBarbary,
    MakeBucaneer,
    Toggle,
    GodsQuery,
}

impl FromStr for InstructionType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(Self::MakeBucaneer),
            "E" => Ok(Self::MakeBarbary),
            "I" => Ok(Self::Toggle),
            "S" => Ok(Self::GodsQuery),
            _ => panic!("Invalid query type"),
        }
    }
}

#[derive(Clone, Copy)]
enum Pirate {
    Barbary,
    Bucaneer,
}

#[derive(Clone)]
enum PiratesTree {
    SinglePirate(Pirate),
    Join {
        left: Rc<PiratesTree>,
        right: Rc<PiratesTree>,
        flipped: bool,
        size: usize,
        num_bucaneers: usize,
    },
}

impl PiratesTree {
    fn from_string(string: &str) -> Self {
        match string.len().cmp(&1) {
            Ordering::Equal => {
                let c = string.chars().next().unwrap();
                PiratesTree::SinglePirate(Pirate::from_char(c).unwrap())
            }
            Ordering::Less => panic!("Invalid string"),
            Ordering::Greater => Self::join(
                Self::from_string(&string[..string.len() / 2]),
                Self::from_string(&string[string.len() / 2..]),
            ),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::SinglePirate(_) => 1,
            Self::Join { size, .. } => *size,
        }
    }

    fn num_bucaneers(&self) -> usize {
        match self {
            Self::SinglePirate(Pirate::Bucaneer) => 1,
            Self::SinglePirate(Pirate::Barbary) => 0,
            &Self::Join {
                num_bucaneers,
                flipped,
                size,
                ..
            } => {
                if flipped {
                    size - num_bucaneers
                } else {
                    num_bucaneers
                }
            }
        }
    }

    fn join(left_pirates: PiratesTree, right_pirates: PiratesTree) -> Self {
        let size = left_pirates.len() + right_pirates.len();
        let num_bucaneers = left_pirates.num_bucaneers() + right_pirates.num_bucaneers();
        Self::Join {
            left: Rc::new(left_pirates),
            right: Rc::new(right_pirates),
            flipped: false,
            size,
            num_bucaneers,
        }
    }

    fn replicate(&self, n: usize) -> Self {
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

    fn from_sections(sections: Vec<PiratesTree>) -> Self {
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
        let PiratesTree::Join { left, right,  .. } = self else {
            panic!("Weird bounds");
        };
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
    fn set(&self, range: RangeInclusive<usize>, pirate_type: Pirate) -> Self {
        self.recurse(
            range,
            &|tree: &PiratesTree| tree.clone(),
            &|tree: &PiratesTree| Self::SinglePirate(pirate_type).replicate(tree.len()),
        )
    }

    fn toggle(&self, range: RangeInclusive<usize>) -> PiratesTree {
        self.recurse(range, &|tree| tree.clone(), &|tree| {
            let mut tree = tree.clone();
            tree.flip();
            tree
        })
    }

    fn count_bucaneers(&self, range: RangeInclusive<usize>) -> usize {
        self.recurse(range, &|_tree| 0, &|tree| {
            let result = tree.num_bucaneers();
            eprintln!("{} {}", tree.to_string(), result);
            result
        })
    }

    fn flip(&mut self) {
        match self {
            PiratesTree::SinglePirate(pirate) => *pirate = pirate.opposite(),
            PiratesTree::Join { flipped, .. } => *flipped = !*flipped,
        }
    }

    fn to_string(&self) -> String {
        match self {
            PiratesTree::SinglePirate(pirate) => pirate.to_string(),
            PiratesTree::Join {
                left,
                right,
                flipped,
                ..
            } => {
                let mut result = left.to_string();
                result.extend(right.to_string().chars());
                if *flipped {
                    result = result
                        .chars()
                        .map(|c| {
                            Pirate::from_char(c).map_or(c, |pirate| pirate.opposite().to_char())
                        })
                        .collect();
                }
                format!("({})", result)
            }
        }
    }
}

impl Pirate {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Self::Bucaneer),
            '0' => Some(Self::Barbary),
            _ => None,
        }
    }

    fn opposite(&self) -> Pirate {
        match self {
            Self::Bucaneer => Self::Barbary,
            Self::Barbary => Self::Bucaneer,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Pirate::Barbary => '0',
            Pirate::Bucaneer => '1',
        }
    }

    fn to_string(&self) -> String {
        self.to_char().to_string()
    }
}

trait Append {
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
