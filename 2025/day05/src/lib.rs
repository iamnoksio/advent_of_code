pub fn fresh_count(fresh_ids: &Vec<(usize, usize)>, ids: &Vec<usize>) -> usize {
    ids.iter()
        .filter(|&id| {
            fresh_ids
                .iter()
                .any(|(start, end)| id >= start && id <= end)
        })
        .count()
}

pub fn fresh_ingredients_range_count(mut fresh_ids: Vec<(usize, usize)>) -> usize {
    if fresh_ids.is_empty() {
        return 0;
    }

    fresh_ids.sort_unstable_by_key(|(start, _)| *start);

    let mut total = 0;

    let (mut c_start, mut c_end) = fresh_ids[0];

    for (n_start, n_end) in fresh_ids.into_iter().skip(1) {
        if n_start <= c_end.saturating_add(1) {
            c_end = c_end.max(n_end);
        } else {
            total += c_end - c_start + 1;
            c_start = n_start;
            c_end = n_end;
        }
    }

    total + (c_end - c_start + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one_fresh_count() {
        let fresh_ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let available_ids = vec![1, 5, 8, 11, 17, 32];

        let result = fresh_count(&fresh_ranges, &available_ids);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two_range_count() {
        let fresh_ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];

        let result = fresh_ingredients_range_count(fresh_ranges);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(fresh_ingredients_range_count(vec![]), 0);

        let contiguous = vec![(1, 2), (3, 4)];
        assert_eq!(fresh_ingredients_range_count(contiguous), 4);

        let overlapping = vec![(1, 10), (2, 5)];
        assert_eq!(fresh_ingredients_range_count(overlapping), 10);
    }
}
