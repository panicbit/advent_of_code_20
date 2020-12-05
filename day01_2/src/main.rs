#[macro_use] extern crate aoc;

#[aoc(2020, 01, 2)]
fn main(input: &str) -> i32 {
    let entries = input
        .split_whitespace()
        .map(|entry| entry.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    
    for (i, entry1) in entries.iter().enumerate() {
        for (i, entry2) in entries[i+1..].iter().enumerate() {
            for entry3 in &entries[i+1..] {
                let sum = entry1 + entry2 + entry3;

                if sum == 2020 {
                    let product = entry1 * entry2 * entry3;

                    return product
                }
            }
        }
    }

    panic!("no result found")
}
