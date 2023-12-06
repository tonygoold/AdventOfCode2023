use std::time::Instant;

use regex::Regex;

use aoc::{input_arg, read_lines};
use aoc::range_map::{RangeMap, RangeMapChain};

pub fn main() {
    let mut lines = read_lines(&input_arg());

    let seed_line = lines.next().expect("Did not find starting line");
    if !seed_line.starts_with("seeds: ") {
        panic!("Did not find starting line");
    }
    let seed_strs = seed_line[7..].split_ascii_whitespace();
    let mut seeds = seed_strs.map(|s|
         s.parse::<usize>().expect("Invalid seed")
    );
    let mut seed_ranges: Vec<_> = Vec::new();
    while let Some(start) = seeds.next() {
        let len = seeds.next().expect("Unbalanced seed ranges");
        seed_ranges.push(start..start+len);
    }

    let map_re = Regex::new(r"^(\w+)-to-(\w+) map:$")
        .expect("Failed to compile map regex");
    let mut source = String::new();
    let mut dest = String::new();
    let mut chain = RangeMapChain::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some(captures) = map_re.captures(&line) {
            source = captures[1].to_owned();
            dest = captures[2].to_owned();
            continue;
        }

        let map: RangeMap = line.parse().expect("Failed to parse range map");
        chain.add(&source, &dest, map);
    }

    let locations = seed_ranges.iter().flat_map(|range| {
        chain.map_range("seed", range)
    });
    let min_location = locations.min()
        .expect("No locations");
    println!("The minimum location is {}", min_location);
}
