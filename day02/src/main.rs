#[macro_use] extern crate aoc;
use utils::*;

#[aoc(2020, 02, 1)]
fn main(input: &str) -> usize {
    input
        .lines()
        .map(Password::parse)
        .filter(|pass| pass.is_valid())
        .count()
}

struct Password<'a> {
    min: usize,
    max: usize,
    char: char,
    text: &'a str,
}

impl<'a> Password<'a> {
    fn parse(line: &'a str) -> Self {
        let caps = re!(r"(?x)
            (?P<min>\d+)-(?P<max>\d+)\s
            (?P<char>\w):\s
            (?P<text>.*)",
            line
        );

        Self {
            min: caps.parse::<usize>("min"),
            max: caps.parse::<usize>("max"),
            char: caps.parse::<char>("char"),
            text: caps.str("text"),
        }
    }

    fn is_valid(&self) -> bool {
        let char_count = self.text.chars().filter(|c| *c == self.char).count();

        self.min <= char_count && char_count <= self.max
    }
}
