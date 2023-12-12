use std::collections::{HashMap, VecDeque};
use std::ops::Index;

use super::grid;
use super::grid::Grid;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidChar,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pipe {
    NS, // |
    EW, // -
    NE, // L
    NW, // J
    SW, // 7
    SE, // F
    Start, // S
    Empty, // .
}

impl Pipe {
    pub fn exit(&self, dir: Direction) -> Option<Direction> {
        use Direction::{North, East, South, West};
        use Pipe::*;
        match *self {
            NS => match dir {
                North => Some(North),
                South => Some(South),
                _ => None,
            },
            EW => match dir {
                East => Some(East),
                West => Some(West),
                _ => None,
            },
            NE => match dir {
                South => Some(East),
                West => Some(North),
                _ => None,
            },
            NW => match dir {
                South => Some(West),
                East => Some(North),
                _ => None,
            },
            SW => match dir {
                North => Some(West),
                East => Some(South),
                _ => None,
            },
            SE => match dir {
                North => Some(East),
                West => Some(South),
                _ => None,
            },
            // Start is special-cased
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        use Pipe::*;
        match *self {
            NS => '|',
            EW => '-',
            NE => '└',
            NW => '┘',
            SW => '┐',
            SE => '┌',
            Start => 'S',
            Empty => '.',
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::NS),
            '-' => Ok(Pipe::EW),
            'L' => Ok(Pipe::NE),
            'J' => Ok(Pipe::NW),
            '7' => Ok(Pipe::SW),
            'F' => Ok(Pipe::SE),
            'S' => Ok(Pipe::Start),
            '.' => Ok(Pipe::Empty),
            _ => Err(Self::Error::InvalidChar),
        }
    }
}

pub struct Matrix {
    grid: Grid<Pipe>,
    // (row, col) -> distance
    visited: HashMap<(usize, usize), usize>,
}

impl Matrix {
    pub fn new(char_grid: &Grid<char>) -> Self {
        let grid: Grid<Pipe> = char_grid.map(|_, cell| {
            Pipe::try_from(*cell).expect("Invalid char")
        });
        Self { grid, visited: HashMap::new() }
    }

    pub fn size(&self) -> (usize, usize) {
        self.grid.size()
    }

    pub fn start(&self) -> (usize, usize) {
        let (row, col, _) = self.grid.iter()
            .find(|(_, _, &pipe)| pipe == Pipe::Start)
            .expect("No start position found");
        (row, col)
    }

    // (row, col, dist)
    pub fn furthest(&mut self) -> (usize, usize, usize) {
        let (rows, cols) = self.size();
        // (row, col, outbound direction, distance)
        let mut queue: VecDeque<(usize, usize, Direction, usize)> = VecDeque::new();
        let (row, col) = self.start();
        if row > 0 {
            queue.push_back((row - 1, col, Direction::North, 0));
        }
        queue.push_back((row + 1, col, Direction::South, 0));
        if col > 0 {
            queue.push_back((row, col - 1, Direction::West, 0));
        }
        queue.push_back((row, col + 1, Direction::East, 0));
        self.visited.insert((row, col), 0);
        let row_range = 0..rows;
        let col_range = 0..cols;
        while let Some((row, col, dir, dist)) = queue.pop_front() {
            if !row_range.contains(&row) || !col_range.contains(&col) {
                continue;
            } else if self.visited.contains_key(&(row, col)) {
                continue;
            }
            let pipe = self.grid[(row, col)];
            if let Some(out_dir) = pipe.exit(dir) {
                match out_dir {
                    Direction::North => if row > 0 { queue.push_back((row - 1, col, out_dir, dist + 1)) }
                    Direction::East => queue.push_back((row, col + 1, out_dir, dist + 1)),
                    Direction::South => queue.push_back((row + 1, col, out_dir, dist + 1)),
                    Direction::West => if col > 0 { queue.push_back((row, col - 1, out_dir, dist + 1)) },
                }
                self.visited.insert((row, col), dist + 1);
            }
        }
        let max = self.visited.iter().max_by_key(|(_, &dist)| dist)
            .expect("No nodes visited");
        ((*max.0).0, (*max.0).1, *max.1)
    }

    pub fn iter(&self) -> MatrixIter {
        MatrixIter { iter: self.grid.iter() }
    }

    pub fn visited(&self, row: usize, col: usize) -> bool {
        self.visited.contains_key(&(row, col))
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = Pipe;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[(index.0, index.1)]
    }
}

pub struct MatrixIter <'a> {
   iter: grid::Iter<'a, Pipe>,
}

impl <'a> Iterator for MatrixIter <'a> {
    type Item = (usize, usize, &'a Pipe);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
