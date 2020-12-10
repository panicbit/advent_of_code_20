#[macro_use] extern crate aoc;

use utils::*;
use std::collections::{BTreeMap, BTreeSet};

#[aoc(2020, 07, 1)]
fn main(input: &str) -> usize {
    let bag_specs = input
        .lines()
        .map(BagSpec::parse)
        .collect::<Vec<_>>();

    let container_lut = create_container_lookup_table(&bag_specs);

    let mut results = BTreeSet::new();
    let mut to_check = vec!["shiny gold"];

    while let Some(bag) = to_check.pop() {
        if bag != "shiny gold" {
            results.insert(bag);
        }

        let containers = container_lut.get(bag);

        if let Some(containers) = containers {
            to_check.extend(containers);
        }
    }

    results.len()
}

fn create_container_lookup_table<'a>(bag_specs: &'a [BagSpec]) -> BTreeMap<&'a str, BTreeSet<&'a str>> {
    let mut container_lut = BTreeMap::new();

    for bag_spec in bag_specs {
        for &content in bag_spec.contents.keys() {
            container_lut
                .entry(content)
                .or_insert(BTreeSet::new())
                .insert(bag_spec.name);
        }
    }

    container_lut
}

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
struct BagSpec<'a> {
    name: &'a str,
    contents: BTreeMap<&'a str, usize>,
}

impl<'a> BagSpec<'a> {
    fn parse(spec: &'a str) -> Self {
        let caps = re!(r"^(?P<name>.*?) bags contain (?P<contents>.*?)\.$", spec);
        let name = caps.str("name");
        let contents = caps.str("contents")
            .split(", ")
            .filter_map(|content| {
                if content == "no other bags" {
                    return None;
                }

                let caps = re!(r"(?P<amount>\d+) (?P<name>.*?) bags?", content);
                let amount = caps.parse::<usize>("amount");
                let name = caps.str("name");

                Some((name, amount))
            })
            .collect::<BTreeMap<_, _>>();

        Self {
            name,
            contents,
        }
    }
}

#[test]
fn bag_spec_parses() {
    let spec = "plaid aqua bags contain 1 posh violet bag, 5 pale yellow bags, 4 bright salmon bags.";
    let bag_spec = BagSpec::parse(spec);

    assert_eq!(bag_spec, BagSpec {
        name: "plaid aqua",
        contents: maplit::btreemap! {
            "posh violet" => 1,
            "pale yellow" => 5,
            "bright salmon" => 4,
        }
    })
}

#[test]
fn bag_spec_no_bags_parses() {
    let spec = "faded blue bags contain no other bags.";
    let bag_spec = BagSpec::parse(spec);

    assert_eq!(bag_spec, BagSpec {
        name: "faded blue",
        contents: maplit::btreemap! {}
    })
}
