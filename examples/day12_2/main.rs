use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;

use adventofcode_2020::error::MyError;

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
    waypoint: (isize, isize),
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0, waypoint: (10, 1) }
    }

    fn apply(&mut self, cmd: &Cmd) {
        match *cmd {
            Cmd::N(n) => self.waypoint.1 += n as isize,
            Cmd::S(n) => self.waypoint.1 -= n as isize,
            Cmd::E(n) => self.waypoint.0 += n as isize,
            Cmd::W(n) => self.waypoint.0 -= n as isize,

            Cmd::L(_) | Cmd::R(_) => self.rotate(cmd),

            Cmd::F(n) => {
                self.x += self.waypoint.0 * n as isize;
                self.y += self.waypoint.1 * n as isize;
            }

            Cmd::None => ()
        }
    }

    // https://www.wikiwand.com/en/Rotation_matrix
    fn rotate(&mut self, dir: &Cmd) {
        match *dir {
            Cmd::L(n) => {
                self.waypoint = rotate(self.waypoint, n as isize);
            }
            Cmd::R(n) => {
                self.waypoint = rotate(self.waypoint, -(n as isize));
            }
            _ => ()
        }
    }

    fn get_distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}


fn rotate(input: (isize, isize), deg: isize) -> (isize, isize) {
    let x = input.0 as f64;
    let y = input.1 as f64;
    let deg = deg as f64 / 180.0 * std::f64::consts::PI;

    let r_x = x * deg.cos() - y * deg.sin();
    let r_y = x * deg.sin() + y * deg.cos();

    // Important: -0.999999 as isize gives 0;
    (r_x.round() as isize, r_y.round() as isize)
}

#[derive(Debug)]
enum Cmd {
    N(usize),
    S(usize),
    E(usize),
    W(usize),

    L(usize),
    R(usize),

    F(usize),

    None,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/day12_2/input.txt")?;

    let position = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<Cmd>().unwrap_or(Cmd::None))
        .fold(Position::new(), |mut position, next| {
            println!("{:?}, next->{:?}", position, next);
            position.apply(&next);
            println!("{:?}", position);
            position
        });

    println!("{:?}, distance: {}", position, position.get_distance());
    Ok(())
}

impl FromStr for Cmd {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, value) = s.split_at(1);
        let value = value.parse::<usize>()?;
        match cmd {
            "N" => Ok(Cmd::N(value)),
            "S" => Ok(Cmd::S(value)),
            "E" => Ok(Cmd::E(value)),
            "W" => Ok(Cmd::W(value)),
            "L" => Ok(Cmd::L(value)),
            "R" => Ok(Cmd::R(value)),
            "F" => Ok(Cmd::F(value)),
            _ => Err(Box::new(MyError::new("Not a Cmd")))
        }
    }
}