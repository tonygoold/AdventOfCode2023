use aoc::{input_arg, read_lines};

fn extract_digits(s: &str) -> Vec<u32> {
    let names = [
        "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine"
    ];
    let mut digits: Vec<u32> = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            digits.push(d);
            continue;
        }
        let substr = &s[i..];
        for (d, name) in names.iter().enumerate() {
            if substr.starts_with(*name) {
                digits.push(d as u32);
                break;
            }
        }
    }
    digits
}

fn main() {
    let lines = read_lines(&input_arg());
    let digits: Vec<_> = lines.map(|l| extract_digits(&l)).collect();
    let sum: u32 = digits.iter().map(
        |ds| 10 * ds.first().expect("no digits") + ds.last().expect("no digits")
    ).sum();
    println!("The sum is {}", sum);
}
