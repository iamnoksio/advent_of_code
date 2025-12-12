use std::num::ParseIntError;

pub fn try_validating_part1(input: &str) -> Result<u64, ParseIntError> {
    let ranges = input.split(',');
    let mut sum_invalid = 0;

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].parse::<u64>()?;
        let end = parts[1].parse::<u64>()?;

        for id in start..=end {
            if !is_invalid_part1(id) {
                continue;
            }
            sum_invalid += id;
        }
    }
    Ok(sum_invalid)
}

fn is_invalid_part1(id: u64) -> bool {
    let s = id.to_string();
    let n = s.len();

    if n & 1 != 0 {
        return false;
    }

    let half = n / 2;
    &s[..half] == &s[half..]
}

pub fn try_validating_part2(input: &str) -> Result<u64, ParseIntError> {
    let ranges = input.split(',');
    let mut sum_invalid = 0;

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].parse::<u64>()?;
        let end = parts[1].parse::<u64>()?;

        for id in start..=end {
            if !is_invalid_part2(id) {
                continue;
            }
            sum_invalid += id;
        }
    }
    Ok(sum_invalid)
}

fn is_invalid_part2(id: u64) -> bool {
    let s = id.to_string();
    let n = s.len();

    for size in 1..=n / 2 {
        if n % size != 0 {
            continue;
        }
        let chunk = &s[0..size];
        let repeat_count = n / size;

        if repeat_count >= 2 && chunk.repeat(repeat_count) == s {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::{try_validating_part1, try_validating_part2};

    #[test]
    fn test_invalid_id_sequences_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862";
        let result = try_validating_part1(input).unwrap();
        assert_eq!(result, 1_227_775_554);
    }

    #[test]
    fn test_invalid_id_sequences_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = try_validating_part2(input).unwrap();
        assert_eq!(result, 4_174_379_265);
    }
}
