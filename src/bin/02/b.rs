use aoc::{input_arg, read_lines};

use aoc::cube_bag::Game;

fn main() {
    let lines = read_lines(&input_arg());
    let games: Vec<Game> = lines.map(|line| line.parse())
        .collect::<Result<Vec<_>,_>>()
        .expect("Invalid input");
    let powers = games.iter().map(|game| {
        game.max_red() * game.max_green() * game.max_blue()
    });
    println!("The sum of game powers is {}", powers.sum::<usize>());
}
