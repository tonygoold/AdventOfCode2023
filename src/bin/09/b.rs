use aoc::{input_arg, read_int_rows};
use aoc::oasis::Sequence;

pub fn main() {
    let mut seqs = read_int_rows(&input_arg())
        .into_iter()
        .map(|mut ns| {
            ns.reverse();
            ns.iter_mut().for_each(|n| *n *= -1);
            Sequence::new(ns)
        })
        .collect::<Vec<_>>();
    let mut vals = seqs.iter_mut()
        .map(|seq| seq.advance())
        .sum::<isize>();
    vals *= -1;
    println!("The sum of new values is {}", vals);
}
