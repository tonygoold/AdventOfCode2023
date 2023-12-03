use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    Red,
    Green,
    Blue
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    WrongFormat,
    InvalidColour,
    InvalidNumber,
    InternalError,
}

pub struct Selection {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

pub struct Game {
    id: usize,
    selections: Vec<Selection>,
}

impl Game {
    pub fn new(id: usize) -> Self {
        Self { id, selections: Vec::new() }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn max_red(&self) -> usize {
        self.selections.iter().map(|s| s.red).max().unwrap_or_default()
    }

    pub fn max_green(&self) -> usize {
        self.selections.iter().map(|s| s.green).max().unwrap_or_default()
    }

    pub fn max_blue(&self) -> usize {
        self.selections.iter().map(|s| s.blue).max().unwrap_or_default()
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_re = Regex::new(r"^Game (\d+): (.+)$")
            .map_err(|_| Self::Err::InternalError)?;
        let captures = line_re.captures(s)
            .ok_or(Self::Err::WrongFormat)?;
        let id: usize = captures[1].parse()
            .map_err(|_| Self::Err::InvalidNumber)?;

        let cube_re = Regex::new(r"(\d+) (red|green|blue)")
            .map_err(|_| Self::Err::InternalError)?;
        let rounds = captures[2].split("; ");
        let selections = rounds.map(|round| {
            let cubes = round.split(", ");
            let mut red: usize = 0;
            let mut green: usize = 0;
            let mut blue: usize = 0;
            for cube in cubes {
                let captures = cube_re.captures(cube)
                    .ok_or(Self::Err::WrongFormat)?;
                let count: usize = captures[1].parse()
                    .map_err(|_| Self::Err::InvalidNumber)?;
                match &captures[2] {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => return Err(Self::Err::InvalidColour),
                }
            }
            Ok(Selection { red, green, blue })
        }).collect::<Result<Vec<_>,_>>()?;

        Ok(Self { id, selections })
    }
}
