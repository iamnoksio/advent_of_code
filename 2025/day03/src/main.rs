use day03::{add_decimal_strings, biggest_12_digit_joltage, biggest_numbers_from_tens};
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let reader = util::file_read("src/input.txt")?;
    let mut total_tens: u16 = 0;
    let mut total_twelves = String::from("0");

    for result in reader.lines() {
        let line = result?;
        let number_tens = biggest_numbers_from_tens(&line);
        let number_twelves = biggest_12_digit_joltage(&line);

        total_twelves = add_decimal_strings(&total_twelves, &number_twelves);
        total_tens += number_tens as u16;
    }

    println!("Total (10): {total_tens}");
    println!("Total (12): {total_twelves}");

    Ok(())
}
