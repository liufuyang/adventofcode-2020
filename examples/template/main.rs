use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/template/input.txt")?;
    let numbers = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap() * 2)
        .collect::<Vec<_>>();

    println!("{:?}", numbers);
    Ok(())
}