pub enum Rotation {
    Left,
    Right,
}
impl Rotation {
    pub fn from_str(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            _ => Self::Left,
        }
    }
}

pub fn rotate(index: u8, number: u16, rotation: &Rotation) -> u8 {
    let steps = (number % 100) as i16;
    let index = index as i16;

    match rotation {
        Rotation::Left => (index - steps).rem_euclid(100) as u8,
        Rotation::Right => (index + steps).rem_euclid(100) as u8,
    }
}
pub fn rotate_and_count(index: u8, number: u16, rotation: &Rotation) -> (u8, u32) {
    let mut index = index as i32;
    let mut zeros = 0;

    let steps = number as i32;
    for _ in 0..steps {
        index = match rotation {
            Rotation::Right => (index + 1) % 100,
            Rotation::Left => (index - 1 + 100) % 100,
        };
        if index == 0 {
            zeros += 1;
        }
    }

    (index as u8, zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_sequence() {
        let mut index = 50u8;

        index = rotate(index, 68, &Rotation::Left);
        assert_eq!(index, 82);

        index = rotate(index, 30, &Rotation::Left);
        assert_eq!(index, 52);

        index = rotate(index, 48, &Rotation::Right);
        assert_eq!(index, 0);

        index = rotate(index, 5, &Rotation::Left);
        assert_eq!(index, 95);

        index = rotate(index, 60, &Rotation::Right);
        assert_eq!(index, 55);

        index = rotate(index, 55, &Rotation::Left);
        assert_eq!(index, 0);

        index = rotate(index, 1, &Rotation::Left);
        assert_eq!(index, 99);

        index = rotate(index, 99, &Rotation::Left);
        assert_eq!(index, 0);

        index = rotate(index, 14, &Rotation::Right);
        assert_eq!(index, 14);

        index = rotate(index, 82, &Rotation::Left);
        assert_eq!(index, 32);
    }

    #[test]
    fn test_rotate_and_count_sequence() {
        let mut index = 50u8;
        let mut total_zeros = 0u32;

        let (new_index, count) = rotate_and_count(index, 68, &Rotation::Left);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 82);

        let (new_index, count) = rotate_and_count(index, 30, &Rotation::Left);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 52);

        let (new_index, count) = rotate_and_count(index, 48, &Rotation::Right);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 0);

        let (new_index, count) = rotate_and_count(index, 5, &Rotation::Left);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 95);

        let (new_index, count) = rotate_and_count(index, 60, &Rotation::Right);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 55);

        let (new_index, count) = rotate_and_count(index, 55, &Rotation::Left);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 0);

        let (new_index, count) = rotate_and_count(index, 1, &Rotation::Left);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 99);

        let (new_index, count) = rotate_and_count(index, 99, &Rotation::Left);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 0);

        let (new_index, count) = rotate_and_count(index, 14, &Rotation::Right);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 14);

        let (new_index, count) = rotate_and_count(index, 82, &Rotation::Left);
        index = new_index;
        total_zeros += count;
        assert_eq!(index, 32);

        assert_eq!(total_zeros, 6);
    }
}
