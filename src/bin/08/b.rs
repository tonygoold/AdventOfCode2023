use gcd::Gcd;

use aoc::{input_arg, read_lines};
use aoc::map_route::{Path, Routes};

pub fn main() {
    let mut lines = read_lines(&input_arg());

    let path = lines.next().expect("No path instructions");
    let mut routes = Routes::new();
    for line in lines {
        if !line.is_empty() {
            routes.insert_str(&line);
        }
    }
    let nodes = routes.get_nodes();
    let paths = nodes.iter()
        .filter(|s| s.ends_with("A"))
        .map(|s| Path::new(&routes, &path, s))
        .collect::<Vec<_>>();

    let ends = paths.into_iter().flat_map(|mut path| path.steps_to_z());
    let lcm = ends.reduce(|acc, end| (acc * end) / acc.gcd(end))
        .expect("No solution");
    println!("It takes {} steps to reach all ends simultaneously", lcm);
}
