use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn Error>> {
    let required_fields: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|f| f.to_string())
        .collect();

    let file = File::open("./examples/day4/input.txt")?;
    let mut v: Vec<String> = Vec::new();
    v.push("".into());
    let passports: Vec<HashSet<String>> = BufReader::new(file)
        .lines()
        .fold(v, |mut acc, line| {
            let line = line.unwrap_or("".into());
            if line.is_empty() {
                acc.push("".into());
                return acc;
            }
            let mut last = acc.pop().unwrap();
            if !last.is_empty() {
                last.push_str(" ");
            }
            last.push_str(line.as_str());
            acc.push(last);
            acc
        })
        .into_iter()
        .map(|passport: String| {
            let fields_vec: Vec<String> = passport
                .split_whitespace()
                .filter(|field| field.len() >= 5)
                .map(|field| to_valid_key(field))
                .collect();
            HashSet::from_iter(fields_vec)
        })
        .filter(|set| required_fields.is_subset(set))
        .collect();

    println!("Result: {:?}", passports.len());

    Ok(())
}

fn to_valid_key(field: &str) -> String {
    let key = &field[0..3];
    let value = &field[4..field.len()];
    let invalid: String = "INVALID".into();
    match key {
        "byr" => {
            if value.len() != 4 {
                return invalid;
            }
            let year = value.parse().unwrap_or(0u32);
            if year < 1920 || year > 2002 {
                return invalid;
            }
            key.to_string()
        }
        "iyr" => {
            if value.len() != 4 {
                return invalid;
            }
            let year = value.parse().unwrap_or(0u32);
            if year < 2010 || year > 2020 {
                return invalid;
            }
            key.to_string()
        }
        "eyr" => {
            if value.len() != 4 {
                return invalid;
            }
            let year = value.parse().unwrap_or(0u32);
            if year < 2020 || year > 2030 {
                return invalid;
            }
            key.to_string()
        }
        "hgt" => {
            let (n, unit) = value.split_at(value.len() - 2);
            match unit {
                "cm" => {
                    let h = n.parse().unwrap_or(0usize);
                    if h < 150 || h > 193 {
                        return invalid;
                    }
                    key.to_string()
                }
                "in" => {
                    let h = n.parse().unwrap_or(0usize);
                    if h < 59 || h > 76 {
                        return invalid;
                    }
                    key.to_string()
                }
                _ => invalid,
            }
        }
        "hcl" => {
            if value.chars().count() != 7 {
                return invalid;
            }
            if value.chars().nth(0).unwrap_or('X') != '#' {
                return invalid;
            }
            for i in 1..7usize {
                if !vec![
                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                ]
                .contains(&value.chars().nth(i).unwrap_or('X'))
                {
                    return invalid;
                }
            }
            key.to_string()
        }
        "ecl" => {
            if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                return invalid;
            }
            key.to_string()
        }
        "pid" => {
            if value.chars().count() != 9 {
                return invalid;
            }
            for i in 0..9usize {
                if !vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
                    .contains(&value.chars().nth(i).unwrap_or('X'))
                {
                    return invalid;
                }
            }
            key.to_string()
        }
        "cid" => key.to_string(),
        _ => invalid,
    }
}
