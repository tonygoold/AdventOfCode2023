use aoc::{input_arg, read_lines};
use aoc::map_route::Routes;

pub fn main() {
    let mut lines = read_lines(&input_arg());

    let path = lines.next().expect("No path instructions");
    let mut routes = Routes::new();
    for line in lines {
        if !line.is_empty() {
            routes.insert_str(&line);
        }
    }

    let steps = routes.trace(&path);
    println!("It takes {} steps to reach ZZZ", steps);
}
