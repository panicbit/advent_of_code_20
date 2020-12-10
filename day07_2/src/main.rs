#[macro_use] extern crate aoc;

use utils::*;
use std::collections::BTreeMap;

#[aoc(2020, 07, 2)]
fn main(input: &str) -> usize {
    let bag_specs = input
        .lines()
        .map(BagSpec::parse)
        .map(|bag| (bag.name, bag))
        .collect::<BTreeMap<_,_>>();
    let shiny_gold_bag = &bag_specs["shiny gold"];

    let mut count = 0;
    let mut to_check = vec![shiny_gold_bag];

    while let Some(bag) = to_check.pop() {
        if bag.name != "shiny gold" {
            count += 1;
        }

        for (name, &count) in &bag.contents {
            let bag = &bag_specs[name];

            for _ in 0..count {
                to_check.push(bag);
            }
        }
    }

    count
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
