use aoc::{input_arg, read_lines};

fn extract_digits(s: &str) -> Vec<u32> {
    s.chars().filter_map(|c| c.to_digit(10))
        .collect()
}

fn main() {
    let lines = read_lines(&input_arg());
    let digits: Vec<_> = lines.map(|l| extract_digits(&l)).collect();
    let sum: u32 = digits.iter().map(
        |ds| 10 * ds.first().expect("no digits") + ds.last().expect("no digits")
    ).sum();
    println!("The sum is {}", sum);
}
