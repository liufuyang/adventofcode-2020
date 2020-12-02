use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use recap::Recap;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?x)
    (?P<p1>\d+)
    -
    (?P<p2>\d+)
    \s+
    (?P<letter>[a-z])
    :\s+
    (?P<text>\S+)
  "#)]
struct Input {
    p1: usize,
    p2: usize,
    letter: char,
    text: String,
}

impl Input {
    fn is_valid(&self) -> bool {
        let count = self.text.matches(self.letter).count();
        if count > self.p2 || count < self.p1 {
            false
        } else {
            true
        }
    }

    fn is_pos_valid(&self) -> bool {
        let cond1 = self
            .text
            .chars()
            .nth(self.p1 - 1)
            .map(|c1| c1.eq(&self.letter))
            .unwrap_or(false);
        let cond2 = self
            .text
            .chars()
            .nth(self.p2 - 1)
            .map(|c1| c1.eq(&self.letter))
            .unwrap_or(false);
        cond1 ^ cond2
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/day2/input_example.txt")?;
    let count = BufReader::new(file)
        .lines()
        .flat_map(|line| {
            let element = line.unwrap_or("".into()).parse::<Input>().ok();
            println!("{:?}", element);
            element.filter(|it| it.is_valid())
        })
        .count();
    println!("Result: {}", count);

    // Part two
    let file = File::open("./examples/day2/input.txt")?;
    let count = BufReader::new(file)
        .lines()
        .flat_map(|line| {
            let element = line.unwrap_or("".into()).parse::<Input>().ok();
            println!("{:?}", element);
            element.filter(|it| it.is_pos_valid())
        })
        .count();

    println!("Result: {}", count);
    Ok(())
}
