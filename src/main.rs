use std::{
    convert::Infallible,
    io::{stdin, Read},
    str::FromStr,
};

use crate::{pirate::Pirate, pirates_tree::PiratesTree};

mod append;
mod flipped;
mod normal;
mod pirate;
mod pirates_tree;

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
