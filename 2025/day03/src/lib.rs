pub fn biggest_numbers_from_tens(input: &str) -> u8 {
    let digits: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut best = 0;

    for i in 0..digits.len() {
        let tens = digits[i];
        if let Some(&ones) = digits[i + 1..].iter().max() {
            best = best.max(tens * 10 + ones);
        }
    }

    best
}

pub fn biggest_12_digit_joltage(input: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut remove = input.len().saturating_sub(12);

    for ch in input.chars() {
        while remove > 0 && !stack.is_empty() && *stack.last().unwrap() < ch {
            stack.pop();
            remove -= 1;
        }
        stack.push(ch);
    }

    stack.truncate(12);

    stack.into_iter().collect()
}

pub fn add_decimal_strings(a: &str, b: &str) -> String {
    let mut result = String::new();
    let mut carry: u8 = 0;

    let mut ia = a.chars().rev();
    let mut ib = b.chars().rev();

    loop {
        let da = ia.next().map(|c| c.to_digit(10).unwrap() as u8);
        let db = ib.next().map(|c| c.to_digit(10).unwrap() as u8);

        if da.is_none() && db.is_none() && carry == 0 {
            break;
        }

        let sum = da.unwrap_or(0) + db.unwrap_or(0) + carry;

        result.push(char::from(b'0' + (sum % 10)));
        carry = sum / 10;
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biggest_numbers_from_tens() {
        let input = "987654321111111";
        assert_eq!(biggest_numbers_from_tens(input), 98);

        let input = "811111111111119";
        assert_eq!(biggest_numbers_from_tens(input), 89);

        let input = "234234234234278";
        assert_eq!(biggest_numbers_from_tens(input), 78);

        let input = "818181911112111";
        assert_eq!(biggest_numbers_from_tens(input), 92);
    }

    #[test]
    fn test_biggest_12_digit_joltage() {
        let input = "987654321111111";
        assert_eq!(biggest_12_digit_joltage(input), "987654321111");

        let input = "811111111111119";
        assert_eq!(biggest_12_digit_joltage(input), "811111111119");

        let input = "234234234234278";
        assert_eq!(biggest_12_digit_joltage(input), "434234234278");

        let input = "818181911112111";
        assert_eq!(biggest_12_digit_joltage(input), "888911112111");
    }

    #[test]
    fn test_add_decimal_strings() {
        let a = "357";
        let b = "0";
        assert_eq!(add_decimal_strings(a, b), "357");

        let a = "17196";
        let b = "3121910778619";
        assert_eq!(add_decimal_strings(a, b), "3121910795815");

        let a = "12345678901234567890";
        let b = "98765432109876543210";
        assert_eq!(add_decimal_strings(a, b), "111111111011111111100");
    }
}
