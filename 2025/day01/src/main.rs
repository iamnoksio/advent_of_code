use day01::{Rotation, rotate, rotate_and_count};
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let path = "src/rotation_list.txt";
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut s_index: u8 = 50;
    let mut s_zeros = 0;

    let mut c_index: u8 = 50;
    let mut c_zeros = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let split = line.split_at(1);
        let prefix = split.0;
        let number: u16 = split.1.parse::<u16>().unwrap();
        let rotation = Rotation::from_str(prefix);

        s_index = rotate(s_index, number, &rotation);
        if s_index == 0 {
            s_zeros += 1;
        }

        let (new_index, count) = rotate_and_count(c_index, number, &rotation);
        c_index = new_index;
        c_zeros += count;
    }

    println!("{c_zeros}");
    println!("{s_zeros}");

    Ok(())
}
