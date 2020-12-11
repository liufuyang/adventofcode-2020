use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

use itertools::sorted;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/day10/input.txt")?;
    let numbers = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap());

    let map: HashMap<usize, usize> = HashMap::new();

    let mut sorted_map = sorted(numbers)
        .fold((map, 0usize), |(mut map, prev), next| {
            let diff = next - prev;
            map.entry(diff).and_modify(|v| *v += 1).or_insert(1);
            (map, next)
        }).0;
    sorted_map.entry(3).and_modify(|v| *v += 1).or_insert(1);

    println!("{:?}", sorted_map);
    Ok(())
}