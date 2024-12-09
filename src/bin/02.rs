use std::iter::zip;

advent_of_code::solution!(2);

fn is_safe(levels: &[u32]) -> bool {
    let mut prev_diff: Option<i32> = None;
    let iter1 = levels.iter();
    let mut iter2 = levels.iter();
    iter2.next();
    zip(iter1, iter2).all(|(x, y)| {
        let diff = (*y as i64 - *x as i64) as i32;
        if let Some(prev) = prev_diff {
            if prev < 0 && diff > 0 || prev > 0 && diff < 0 {
                return false;
            }
        }
        let abs_diff = diff.abs();
        if !(1..=3).contains(&abs_diff) {
            return false;
        }
        prev_diff = Some(diff);
        true
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|levels| is_safe(levels))
        .count() as u32;
    Some(res)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
