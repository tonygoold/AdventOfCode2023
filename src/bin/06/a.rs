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
    let times = times_str.split_ascii_whitespace().map(|s| s.parse::<usize>().expect("Invalid number"));
    let dists = dists_str.split_ascii_whitespace().map(|s| s.parse::<usize>().expect("Invalid number"));
    let races: Vec<Race> = times.zip(dists)
        .map(|(time, dist)| Race::new(time, dist))
        .collect();
    let margin: usize = races.iter().map(Race::ways_to_win).product();
    println!("The error margin is {}", margin);
}
