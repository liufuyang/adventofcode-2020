use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()>{

    let file = File::open("./examples/day1/input_example.txt")?;
    let mut map = HashMap::new();

    for line in BufReader::new(file).lines() {
        let num = line.unwrap_or("0".into()).parse::<u32>().unwrap_or(0);
        let counter_part = 2020u32 - num;
        println!("check {} -> {}", num, counter_part);
        match map.get(&(counter_part)) {
            Some(_) => {
                println!("Found 2 sum result: {}", counter_part * num);
                break;
            }
            None => {
                map.insert(num, 1);
            }
        }
    }

    // For 3 sum
    let file = File::open("./examples/day1/input_example.txt")?;
    let mut map = HashMap::new();
    for line in BufReader::new(file).lines() {
        let num = line.unwrap_or("0".into()).parse::<u32>().unwrap_or(0);
        for (k, _) in map.iter() {
            if let Some(counter_part) = 2020u32.checked_sub(num + k) {
                println!("check {}, {} -> {}", num, k, counter_part);
                match map.get(&(counter_part)) {
                    Some(_) => {
                        println!("Found 2 sum result: {}", counter_part * num * k);
                        return Ok(())
                    }
                    None => ()
                }
            }
        }
        map.insert(num, 1);
    }

    Ok(())
}