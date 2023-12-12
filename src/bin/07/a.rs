use aoc::{input_arg, read_lines};
use aoc::camel_cards::Hand;

pub fn main() {
    let mut hands = read_lines(&input_arg())
        .map(|line| Hand::from_str(&line, false))
        .collect::<Result<Vec<_>,_>>()
        .expect("Failed to parse hands");
    hands.sort_unstable();
    let winnings = hands.iter().enumerate()
        .map(|(n, hand)| (n+1) * hand.bid())
        .sum::<usize>();
    println!("The winnings are {}", winnings);
}