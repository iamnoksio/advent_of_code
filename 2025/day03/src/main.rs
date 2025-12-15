use std::io::BufRead;

mod lib;

fn main() -> std::io::Result<()> {
    let path = "src/input.txt";
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut total_tens: u16 = 0;
    let mut total_twelves = String::from("0");

    for result in reader.lines() {
        let line = result?;
        let number_tens = lib::biggest_numbers_from_tens(&line);
        let number_twelves = lib::biggest_12_digit_joltage(&line);

        total_twelves = lib::add_decimal_strings(&total_twelves, &number_twelves);
        total_tens += number_tens as u16;
    }

    println!("Total (10): {total_tens}");
    println!("Total (12): {total_twelves}");

    Ok(())
}
