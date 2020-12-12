use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;

use adventofcode_2020::error::MyError;

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0, direction: Direction::E }
    }

    fn apply(&mut self, cmd: &Cmd) {
        match *cmd {
            Cmd::N(n) => self.y += n as isize,
            Cmd::S(n) => self.y -= n as isize,
            Cmd::E(n) => self.x += n as isize,
            Cmd::W(n) => self.x -= n as isize,

            Cmd::L(_) | Cmd::R(_)  => self.direction.change(cmd),

            Cmd::F(n) => {
                match self.direction {
                    Direction::N => self.apply(&Cmd::N(n)),
                    Direction::S => self.apply(&Cmd::S(n)),
                    Direction::E => self.apply(&Cmd::E(n)),
                    Direction::W => self.apply(&Cmd::W(n)),
                }
            }

            Cmd::None => ()
        }
    }

    fn get_distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn as_vector(&self) -> (isize, isize) {
        match *self {
            Direction::N => (0, 1),
            Direction::S => (0, -1),
            Direction::W => (-1, 0),
            Direction::E => (1, 0),
        }
    }

    fn from_vector( vec: (isize, isize)) -> Direction {
        match vec {
            (0, 1) => Direction::N,
            (0, -1) => Direction::S,
            (-1, 0) => Direction::W,
            (1, 0) => Direction::E,
            _ => Direction::E
        }
    }

    // https://www.wikiwand.com/en/Rotation_matrix
    fn change(&mut self, dir: &Cmd) {
        match *dir {
            Cmd::L(n) => {
               let new_dir =  rotate(self.as_vector(), n as isize);
                *self = Direction::from_vector(new_dir);
            }
            Cmd::R(n) => {
                let new_dir =  rotate(self.as_vector(), -(n as isize));
                *self = Direction::from_vector(new_dir);
            }
            _ => ()
        }
    }
}

fn rotate(input: (isize, isize), deg: isize) -> (isize, isize) {
    let x = input.0 as f64;
    let y = input.1 as f64;
    let deg = deg as f64 / 180.0 * std::f64::consts::PI;

    let r_x = x * deg.cos() - y * deg.sin();
    let r_y = x * deg.sin() + y * deg.cos();

    (r_x as isize, r_y as isize)
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
    let file = File::open("./examples/day12/input.txt")?;

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
    type Err =  Box<dyn Error>;

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