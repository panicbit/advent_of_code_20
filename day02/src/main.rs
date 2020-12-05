#[macro_use] extern crate aoc;

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
        let mut parts = line.split_whitespace();
        let mut range = parts.next().unwrap().split('-');
        let char = parts.next().unwrap().chars().next().unwrap();
        let text = parts.next().unwrap();

        Self {
            min: range.next().unwrap().parse().unwrap(),
            max: range.next().unwrap().parse().unwrap(),
            char,
            text,
        }
    }

    fn is_valid(&self) -> bool {
        let char_count = self.text.chars().filter(|c| *c == self.char).count();

        self.min <= char_count && char_count <= self.max
    }
}
