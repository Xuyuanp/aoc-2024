use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

fn parse_dependencies(input: &str) -> HashMap<u32, HashSet<u32>> {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let nums = line
                .split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (nums[0], nums[1])
        })
        .fold(HashMap::new(), |mut deps, (x, y)| {
            deps.entry(y).or_default().insert(x);
            deps
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    let deps = parse_dependencies(input);

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
            for num in nums {
                if let Some(upstreams) = deps.get(num) {
                    for upstream in upstreams {
                        if nums.contains(upstream) && !visited.contains(upstream) {
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
    let deps = parse_dependencies(input);

    let res = input
        .lines()
        .skip_while(|line| !line.is_empty()) // skip rules
        .skip(1) // skip empty line
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter_map(|nums| {
            let mut new_nums = vec![];
            let mut changed = false;
            let mut queue = VecDeque::from(nums.clone());

            while let Some(num) = queue.pop_front() {
                if let Some(upstreams) = deps.get(&num) {
                    let ok = upstreams
                        .iter()
                        .filter(|upstream| nums.contains(upstream)) // upstream in this update
                        .all(|upstream| new_nums.contains(upstream)); // upstream already visited
                    if !ok {
                        changed = true;
                        queue.push_back(num);
                        continue;
                    }
                }
                new_nums.push(num);
            }

            if changed {
                Some(new_nums[new_nums.len() / 2])
            } else {
                None
            }
        })
        .sum();
    Some(res)
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
        assert_eq!(result, Some(123));
    }
}
