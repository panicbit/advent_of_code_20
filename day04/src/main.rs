#[macro_use] extern crate aoc;

use std::collections::BTreeMap;

#[aoc(2020, 04, 1)]
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
        let is_valid = num_missing_fields == 0;

        if is_valid {
            num_valid_passports += 1;
        }
    }

    num_valid_passports
}
