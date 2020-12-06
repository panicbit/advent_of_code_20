#[macro_use] extern crate aoc;

use std::collections::BTreeMap;
use utils::*;

#[aoc(2020, 04, 2)]
fn main(input: &str) -> i32 {
    let passports = input
        .split("\n\n")
        .map(|passport| passport
            .split_whitespace()
            .map(|field| {
                let mut fields = field.split(':');
                (fields.next().unwrap(), fields.next().unwrap())
            })
            .collect::<BTreeMap<_, _>>()
        );

    let required_fields = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ];

    let mut num_valid_passports = 0;

    for passport in passports {
        let num_present_fields = required_fields
            .iter()
            .filter(|field| passport.contains_key(*field))
            .count();
        let num_missing_fields = required_fields.len() - num_present_fields;
        let has_required_fields = num_missing_fields == 0;


        if !has_required_fields {
            continue
        }

        let conditions = [
            passport["byr"].i32() >= 1920,
            passport["byr"].i32() <= 2002,
            passport["iyr"].i32() >= 2010,
            passport["iyr"].i32() <= 2020,
            passport["eyr"].i32() >= 2020,
            passport["eyr"].i32() <= 2030,
            Height::parse(passport["hgt"]).map(|h| h.is_valid()).unwrap_or(false),
            passport["hcl"].is_match("^#[0-9a-f]{6}$"),
            passport["ecl"].is_match("^amb|blu|brn|gry|grn|hzl|oth$"),
            passport["pid"].is_match(r"^\d{9}$"),
        ];

        let is_valid = conditions.iter().all(|&b| b);

        if !is_valid {
            continue
        }

        num_valid_passports += 1;
    }

    num_valid_passports
}

enum Height {
    CM(usize),
    IN(usize),
}

impl Height {
    fn parse(height: &str) -> Option<Self> {
        let caps = re!(r"(?P<height>\d+)(?P<unit>cm|in)").captures(height)?;
        let height = caps.parse::<usize>("height");

        Some(match caps.str("unit") {
            "cm" => Self::CM(height),
            "in" => Self::IN(height),
            _ => unreachable!(),
        })
    }

    fn is_valid(&self) -> bool {
        match *self {
            Self::CM(height) => 150 <= height && height <= 193,
            Self::IN(height) => 59 <= height && height <= 76,
        }
    }
}
