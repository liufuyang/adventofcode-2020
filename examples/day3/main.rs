use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/day3/input.txt")?;
    let right_n = 1;
    let down_n = 2;
    let count = BufReader::new(file)
        .lines()
        .skip(down_n)
        .step_by(down_n)
        .fold((right_n, 0), |(mut pos, acc), line| {
            let element = line.unwrap_or("".into());
            println!("{:?}", element);
            let n = element.chars().count();
            if pos >= n {
                pos = pos - n;
            }
            match element.chars().nth(pos) {
                Some('#') => (pos + right_n, acc + 1),
                _ => (pos + right_n, acc)
            }
        });
    println!("Result: {:?}", count);
    Ok(())
}

// 84, 195, 70, 70, 47
