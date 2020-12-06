#[macro_use] extern crate aoc;
use std::collections::BTreeSet;

#[aoc(2020, 06, 1)]
fn main(input: &str) -> usize {
    input
    .split("\n\n")
    .map(|group| group
        .lines()
        .flat_map(|answers| answers
            .chars()
        )
        .collect::<BTreeSet<_>>()
        .len()
    )
    .sum()
}
