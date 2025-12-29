use day04::{part1, part2};
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = util::file_read("src/input.txt")?;

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(grid));

    Ok(())
}
