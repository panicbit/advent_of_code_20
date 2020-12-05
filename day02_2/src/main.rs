#[macro_use] extern crate aoc;
use utils::*;

#[aoc(2020, 02, 2)]
fn main(input: &str) -> usize {
    input
        .lines()
        .map(Password::parse)
        .filter(|pass| pass.is_valid())
        .count()
}

struct Password<'a> {
    pos1: usize,
    pos2: usize,
    char: char,
    text: &'a str,
}

impl<'a> Password<'a> {
    fn parse(line: &'a str) -> Self {
        let caps = re!(r"(?x)
            (?P<pos1>\d+)-(?P<pos2>\d+)\s
            (?P<char>\w):\s
            (?P<text>.*)",
            line
        );

        Self {
            pos1: caps.parse::<usize>("pos1") - 1,
            pos2: caps.parse::<usize>("pos2") - 1,
            char: caps.parse::<char>("char"),
            text: caps.str("text"),
        }
    }

    fn is_valid(&self) -> bool {
        let char1 = self.text.chars().nth(self.pos1).unwrap();
        let char2 = self.text.chars().nth(self.pos2).unwrap();

        char1 != char2 && [char1, char2].contains(&self.char)
    }
}
