use std::fs::File;
use std::io::{BufRead, BufReader};

mod lib;

fn main() -> std::io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    println!("Part 1: {}", lib::part1(&grid));
    println!("Part 2: {}", lib::part2(grid));

    Ok(())
}
