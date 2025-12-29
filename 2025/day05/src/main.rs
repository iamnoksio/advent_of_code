use day05::{fresh_count, fresh_ingredients_range_count};
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = util::file_read("src/input.txt")?;

    let mut lines = reader.lines().map(|lines| lines.unwrap());

    let fresh_ids: Vec<(usize, usize)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').expect("invalid pair format!");
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect();

    let ids: Vec<usize> = lines.map(|line| line.parse::<usize>().unwrap()).collect();
    let fresh_count = fresh_count(&fresh_ids, &ids);
    let range_count = fresh_ingredients_range_count(fresh_ids);

    println!("Fresh count: {fresh_count}");
    println!("Range count: {range_count}");
    Ok(())
}
