use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut deps: HashMap<u32, HashSet<u32>> = HashMap::new();

    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums = line
                .split("|")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (nums[0], nums[1])
        })
        .for_each(|(x, y)| {
            let ent = deps.entry(y).or_default();
            ent.insert(x);
        });

    let res = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|nums| {
            let mut visited = HashSet::new();
            let curr_update: HashSet<u32> = HashSet::from_iter(nums.iter().cloned());
            for num in nums {
                if let Some(upstreams) = deps.get(num) {
                    for upstream in upstreams {
                        if curr_update.contains(upstream) && !visited.contains(upstream) {
                            return false;
                        }
                    }
                };
                visited.insert(*num);
            }
            true
        })
        .map(|nums| nums[nums.len() / 2])
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
