use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)").unwrap();
    let res = re
        .captures_iter(input)
        .map(|cap| {
            let first = cap["first"].parse::<u32>().unwrap();
            let second = cap["second"].parse::<u32>().unwrap();
            first * second
        })
        .sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_num = Regex::new(r"\d{1,3}").unwrap();
    let re2 = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    let mut skip = false;
    let mut res = 0;
    for mat in re2.find_iter(input) {
        match mat.as_str() {
            "do()" => skip = false,
            "don't()" => skip = true,
            s => {
                if !skip {
                    let v = re_num
                        .find_iter(s)
                        .map(|mat| mat.as_str().parse::<u32>().unwrap())
                        .product::<u32>();
                    res += v;
                }
            }
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
