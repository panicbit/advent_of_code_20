#[macro_use] extern crate aoc;

#[aoc(2020, 03, 2)]
fn main(input: &str) -> usize {
    let map = Map::parse(input);
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];


    slopes
        .iter()
        .map(|&(x_delta, y_delta)| num_encountered_trees(&map, x_delta, y_delta))
        .product()
}

fn num_encountered_trees(map: &Map, x_delta: usize, y_delta: usize) -> usize {
    let mut x = x_delta;
    let mut y = y_delta;
    let mut encountered_trees = 0;

    while y < map.height() {
        if map.get(x, y) == Location::Tree {
            encountered_trees += 1;
        }

        x += x_delta;
        y += y_delta;
    }

    encountered_trees
}

struct Map {
    locations: Vec<Vec<Location>>
}

impl Map {
    fn parse(map: &str) -> Self {
        let locations = map
            .lines()
            .map(|row| row
                .chars()
                .map(|c| match c {
                    '.' => Location::Open,
                    '#' => Location::Tree,
                    _ => unreachable!(),
                })
                .collect()
            )
            .collect();

        Self {
            locations
        }
    }

    fn height(&self) -> usize {
        self.locations.len()
    }

    fn get(&self, x: usize, y: usize) -> Location {
        let row = &self.locations[y];
        let x = x % row.len();

        row[x]
    }
}

#[derive(Copy,Clone,PartialEq)]
enum Location {
    Open,
    Tree,
}