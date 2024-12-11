use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Direction(i32, i32);

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point(i32, i32);

impl std::ops::Add<Direction> for Point {
    type Output = Point;
    fn add(self, rhs: Direction) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let directions = [
        Direction(-1, 0), // bottom to top
        Direction(0, 1),  // left to right
        Direction(1, 0),  // top to bottom
        Direction(0, -1), // right to left
    ];
    let matrix = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start: Option<Point> = None;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'^' {
                start = Some(Point(i as i32, j as i32));
            }
        }
    }
    let mut visited = HashSet::<Point>::new();

    let mut directions = directions.iter().cycle();

    let mut curr_direction = directions.next().unwrap();
    let mut curr_point = start.unwrap();

    loop {
        visited.insert(curr_point);
        let next_point = curr_point + *curr_direction;
        if next_point.0 < 0
            || next_point.0 >= matrix.len() as i32
            || next_point.1 < 0
            || next_point.1 >= matrix[0].len() as i32
        {
            break;
        }
        if matrix[next_point.0 as usize][next_point.1 as usize] == b'#' {
            curr_direction = directions.next().unwrap();
            continue;
        }

        curr_point = next_point;
    }

    Some(visited.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
