#[macro_use] extern crate aoc;

use std::collections::BTreeSet;
use itertools::Itertools;

#[aoc(2020, 06, 2)]
fn main(input: &str) -> usize {
    input
    .split("\n\n")
    .map(|group| group
        .lines()
        .map(|answers| answers
            .chars()
            .collect::<BTreeSet<_>>()
        )
        .fold1(|answers1, answers2| answers1
            .intersection(&answers2)
            .copied()
            .collect()
        )
        .unwrap()
        .len()
    )
    .sum()
}
