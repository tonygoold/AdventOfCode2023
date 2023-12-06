use std::collections::HashSet;

use aoc::{input_arg, read_lines};

fn main() {
    let lines = read_lines(&input_arg());
    let cards: Vec<(HashSet<usize>, HashSet<usize>)> = lines.map(|s| {
        let i = s.find(':').expect("No colon found");
        let s = &s[(i+1)..];
        let mut sides = s.split('|');
        let left = sides.next().expect("No cards found");
        let right = sides.next().expect("Only one card found");
        let mut left_card = HashSet::new();
        let mut right_card = HashSet::new();
        for n in left.split_ascii_whitespace() {
            left_card.insert(n.parse::<usize>().expect("Invalid number"));
        }
        for n in right.split_ascii_whitespace() {
            right_card.insert(n.parse::<usize>().expect("Invalid number"));
        }
        (left_card, right_card)
    }).collect();

    let mut card_counts: Vec<usize> = Vec::new();
    card_counts.resize(cards.len(), 1);
    for (i, (left, right)) in cards.iter().enumerate() {
        let matches = left.intersection(right).count();
        (1..=matches).for_each(|j| {
            if i+j < card_counts.len() {
                card_counts[i+j] += card_counts[i];
            }
        });
    }
    let sum: usize = card_counts.iter().sum();
    println!("There are {} cards", sum);
}