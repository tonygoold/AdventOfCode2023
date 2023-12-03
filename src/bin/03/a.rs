use std::collections::HashMap;

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

    let mut neighbours: Vec<Coord> = Vec::new();
    neighbours.reserve_exact(8);
    let part_nums = numbers.iter().filter_map(|(n, ps)| {
        ps.iter().find_map(|p| {
            neighbours.clear();
            neighbours.push(Coord::new(p.x - 1, p.y - 1));
            neighbours.push(Coord::new(p.x, p.y - 1));
            neighbours.push(Coord::new(p.x + 1, p.y - 1));
            neighbours.push(Coord::new(p.x - 1, p.y));
            neighbours.push(Coord::new(p.x + 1, p.y));
            neighbours.push(Coord::new(p.x - 1, p.y + 1));
            neighbours.push(Coord::new(p.x, p.y + 1));
            neighbours.push(Coord::new(p.x + 1, p.y + 1));
            let symbol = neighbours.iter().find_map(|p| symbols.get(p));
            symbol.map(|c| (n, *c))
        })
    });
    let sum: usize = part_nums.map(|(n, _)| n).sum();
    println!("The sum of part numbers is {}", sum);
}
