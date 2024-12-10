advent_of_code::solution!(4);

const WORD: [u8; 4] = [b'X', b'M', b'A', b'S'];

struct Direction(i32, i32);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn dfs(
        matrix: &[Vec<u8>],
        i: usize,
        j: usize,
        curr: usize,
        direction: Option<Direction>,
    ) -> u32 {
        if matrix[i][j] != WORD[curr] {
            return 0;
        }
        if curr == WORD.len() - 1 {
            return 1;
        }
        let directions = if let Some(direction) = direction {
            vec![direction]
        } else {
            vec![
                Direction(0, 1),   // up
                Direction(0, -1),  // down
                Direction(1, 0),   // right
                Direction(-1, 0),  // left
                Direction(1, 1),   // up-right
                Direction(-1, -1), // down-left
                Direction(1, -1),  // down-right
                Direction(-1, 1),  // up-left
            ]
        };
        directions
            .into_iter()
            .filter_map(|dir| {
                let x = i as i32 + dir.0;
                let y = j as i32 + dir.1;
                if x >= 0 && x < matrix.len() as i32 && y >= 0 && y < matrix[i].len() as i32 {
                    Some(dfs(matrix, x as usize, y as usize, curr + 1, Some(dir)))
                } else {
                    None
                }
            })
            .sum()
    }

    let mut res = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            res += dfs(&matrix, i, j, 0, None);
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut res = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if matrix[i][j] != b'A' {
                continue;
            }
            match (
                matrix[i - 1][j - 1],
                matrix[i + 1][j + 1],
                matrix[i - 1][j + 1],
                matrix[i + 1][j - 1],
            ) {
                (b'M', b'S', b'M', b'S') => res += 1,
                (b'M', b'S', b'S', b'M') => res += 1,
                (b'S', b'M', b'M', b'S') => res += 1,
                (b'S', b'M', b'S', b'M') => res += 1,
                _ => (),
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
