use aoc::{input_arg, read_int_rows};
use aoc::oasis::Sequence;

pub fn main() {
    let mut seqs = read_int_rows(&input_arg())
        .into_iter()
        .map(Sequence::new)
        .collect::<Vec<_>>();
    let vals = seqs.iter_mut()
        .map(|seq| seq.advance())
        .sum::<isize>();
    println!("The sum of new values is {}", vals);
}
