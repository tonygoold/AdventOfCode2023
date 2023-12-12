use aoc::{input_arg, read_char_grid};
use aoc::pipes::{Matrix, Pipe};

pub fn main() {
    let grid = read_char_grid(&input_arg());
    let mut matrix = Matrix::new(&grid);
    _ = matrix.furthest();
    let (rows, cols) = grid.size();
    let mut empty: usize = 0;
    for row in 0..rows {
        let mut inside = false;
        for col in 0..cols {
            if matrix.visited(row, col) {
                match matrix[(row, col)] {
                    Pipe::NS | Pipe::NW | Pipe::NE => inside = !inside,
                    _ => {},
                }
            } else if inside {
                empty += 1;
            }
        }
    }
    println!("The contained area has {} cells", empty);
}
