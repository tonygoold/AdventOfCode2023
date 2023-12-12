use std::ops::Add;
use aoc::{input_arg, read_lines};
use aoc::race::Race;

pub fn main() {
    let mut lines = read_lines(&input_arg());
    let times_str = lines.next()
        .and_then(|s| s.strip_prefix("Time:").map(|s| s.to_owned()))
        .expect("No Time line");
    let dists_str = lines.next()
        .and_then(|s| s.strip_prefix("Distance:").map(|s| s.to_owned()))
        .expect("No Distance line");
    let time = times_str.split_ascii_whitespace()
        .fold(String::new(), String::add)
        .parse::<usize>()
        .expect("Invalid number");
    let dist = dists_str.split_ascii_whitespace()
        .fold(String::new(), String::add)
        .parse::<usize>()
        .expect("Invalid number");
    let race = Race::new(time, dist);
    let count = race.ways_to_win();
    println!("There are {} ways to win the race", count);
}
