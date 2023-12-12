use aoc::{input_arg, read_char_grid};
use aoc::pipes::Matrix;

pub fn main() {
    let grid = read_char_grid(&input_arg());
    let mut matrix = Matrix::new(&grid);
    let furthest = matrix.furthest();
    println!("The furthest section is {} distance", furthest.2);
}
