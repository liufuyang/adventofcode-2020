use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

use itertools::sorted;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/day10_2/input.txt")?;
    let numbers = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let last = sorted(numbers.clone()).last().unwrap_or(0);
    println!("{:?}", last);

    let mut map: HashMap<usize, usize> = HashMap::new();
    map.insert(0, 1);

    let sorted_map = sorted(numbers)
        .fold(map, |mut map, next| {
            let left_n = next.checked_sub(3).unwrap_or(0);
            let mut count = 0usize;
            for i in left_n..next {
                if let Some(v) = map.get(&i) {
                    count += v;
                }
            }
            map.insert(next, count);
            map
        });

    println!("{:?}", sorted_map);
    println!("{}", sorted_map.get(&last).unwrap_or(&0));
    Ok(())
}