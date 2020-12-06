#[macro_use] extern crate aoc;

use utils::*;
use std::collections::BTreeSet;

#[aoc(2020, 05, 2)]
fn main(input: &str) -> usize {
    let ids = input
        .lines()
        .map(|pass| Pass::parse(pass).id())
        .collect::<BTreeSet<_>>();
    
    for id in &ids {
        if !ids.contains(&(id + 2)) {
            continue
        };

        let my_id = id + 1;

        if ids.contains(&my_id) {
            continue
        }

        return my_id
    }

    unreachable!()
}

struct Pass {
    row: usize,
    column: usize,
}

impl Pass {
    fn parse(pass: &str) -> Self {
        let caps = re!("(?P<row>(F|B){7})(?P<column>(L|R){3})", pass);

        Self {
            row: bin_to_usize(&caps["row"], 'F', 'B'),
            column: bin_to_usize(&caps["column"], 'L', 'R'),
        }
    }

    fn id(&self) -> usize {
        calc_id(self.row, self.column)
    }
}

fn calc_id(row: usize, column: usize) -> usize {
    8 * row + column
}

fn bin_to_usize(bin: &str, zero: char, one: char) -> usize {
    bin
    .chars()
    .map(|c| {
        if c == zero {
            0
        } else if c == one {
            1
        } else {
            unreachable!("unknown bin digit: {}", c)
        }
    })
    .fold(0, |acc, digit| (acc << 1) | digit)
}

#[test]
fn pass_parses() {
    let pass = Pass::parse("FBFBBFFRLR");

    assert_eq!(pass.row, 44);
    assert_eq!(pass.column, 5);
}

#[test]
fn pass_examples() {
    let pass = Pass::parse("BFFFBBFRRR");
    assert_eq!(pass.row, 70);
    assert_eq!(pass.column, 7);
    assert_eq!(pass.id(), 567);

    let pass = Pass::parse("FFFBBBFRRR");
    assert_eq!(pass.row, 14);
    assert_eq!(pass.column, 7);
    assert_eq!(pass.id(), 119);

    let pass = Pass::parse("BBFFBBFRLL");
    assert_eq!(pass.row, 102);
    assert_eq!(pass.column, 4);
    assert_eq!(pass.id(), 820);
}
