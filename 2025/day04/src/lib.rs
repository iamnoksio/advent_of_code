const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part1(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let mut neighbors = 0;

            for (dr, dc) in DIRECTIONS {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                    if grid[nr as usize][nc as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

pub fn part2(mut grid: Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }

                let mut neighbors = 0;

                for (dr, dc) in DIRECTIONS {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        if grid[nr as usize][nc as usize] == '@' {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in &to_remove {
            grid[*r][*c] = '.';
        }

        total_removed += to_remove.len();
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grid() -> Vec<Vec<char>> {
        [
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|row| row.chars().collect())
        .collect()
    }

    #[test]
    fn test_part1_example() {
        let grid = example_grid();
        assert_eq!(part1(&grid), 13);
    }

    #[test]
    fn test_part2_example() {
        let grid = example_grid();
        assert_eq!(part2(grid), 43);
    }
}
