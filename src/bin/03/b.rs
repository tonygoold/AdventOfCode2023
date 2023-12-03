use std::collections::{HashMap, HashSet};

use aoc::{input_arg, read_char_grid};
use aoc::point::Point2D;

type Coord = Point2D<isize>;

fn main() {
    let grid = read_char_grid(&input_arg());
    let mut symbols: HashMap<Coord, char> = HashMap::new();
    let mut cur_number: usize = 0;
    let mut cur_pos: Vec<Coord> = Vec::new();
    let mut numbers: Vec<(usize, Vec<Coord>)> = Vec::new();
    for (row, col, c) in grid.iter() {
        let p = Coord::new(col as isize, row as isize);
        if let Some(n) = c.to_digit(10) {
            cur_number = cur_number * 10 + (n as usize);
            cur_pos.push(p);
        } else {
            if cur_number != 0 {
                numbers.push((cur_number, cur_pos));
                cur_number = 0;
                cur_pos = Vec::new();
            }
            if *c != '.' {
                symbols.insert(p, *c);
            }
        }
    }

    let mut gears: HashMap<Coord, Vec<usize>> = HashMap::new();
    for (n, ps) in numbers.iter() {
        let mut neighbours: HashSet<Coord> = HashSet::new();
        for p in ps.iter() {
            neighbours.insert(Coord::new(p.x - 1, p.y - 1));
            neighbours.insert(Coord::new(p.x, p.y - 1));
            neighbours.insert(Coord::new(p.x + 1, p.y - 1));
            neighbours.insert(Coord::new(p.x - 1, p.y));
            neighbours.insert(Coord::new(p.x + 1, p.y));
            neighbours.insert(Coord::new(p.x - 1, p.y + 1));
            neighbours.insert(Coord::new(p.x, p.y + 1));
            neighbours.insert(Coord::new(p.x + 1, p.y + 1));
        }
        for neighbour in neighbours.iter() {
            if let Some(symbol) = symbols.get(neighbour) {
                if *symbol == '*' {
                    gears.entry(*neighbour).or_default().push(*n);
                }
            }
        }
    };
    let gear_ratios = gears.values().filter_map(|ns| {
        if ns.len() == 2 {
            Some(ns[0] * ns[1])
        } else {
            None
        }
    });
    let sum: usize = gear_ratios.sum();
    println!("The sum of gear ratios is {}", sum);
}
