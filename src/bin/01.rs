use std::iter::zip;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut col1 = vec![];
    let mut col2 = vec![];

    input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|word| word.parse::<u32>().unwrap())
                .collect();
            assert_eq!(numbers.len(), 2);
            (numbers[0], numbers[1])
        })
        .for_each(|(a, b)| {
            col1.push(a);
            col2.push(b);
        });
    col1.sort();
    col2.sort();

    let res = zip(col1, col2).map(|(a, b)| a.abs_diff(b)).sum();

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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
