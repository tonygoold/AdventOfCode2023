use aoc::{input_arg, read_lines};

use aoc::cube_bag::Game;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let lines = read_lines(&input_arg());
    let games: Vec<Game> = lines.map(|line| line.parse())
        .collect::<Result<Vec<_>,_>>()
        .expect("Invalid input");
    let valid_ids = games.iter().filter_map(|game| {
        if game.max_red() <= MAX_RED && game.max_green() <= MAX_GREEN && game.max_blue() <= MAX_BLUE {
            Some(game.id())
        } else {
            None
        }
    });
    println!("The sum of valid game ids is {}", valid_ids.sum::<usize>());
}
