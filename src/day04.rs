use crate::input;

pub fn run() {
    let lines = input::get_lines("day04");

    println!("Part1: {:?}", count_xmas(&lines));
    println!("Part2: {:?}", count_x_mas(&lines));
}

fn count_xmas(lines: &[String]) -> usize {
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let target = ['M', 'A', 'S'];

    let height = matrix.len();
    let width = matrix[0].len();
    let mut result = 0;

    for row in 0..height {
        for col in 0..width {
            if matrix[row][col] != 'X' {
                continue;
            }
            if col > 2 && matrix[row][col - 3..col] == ['S', 'A', 'M'] {
                result += 1;
            }
            if col < width - 3 && matrix[row][col + 1..col + 4] == target {
                result += 1;
            }
            if row > 2 && (1..4).all(|x| matrix[row - x][col] == target[x - 1]) {
                result += 1;
            }
            if row < width - 3 && (1..4).all(|x| matrix[row + x][col] == target[x - 1]) {
                result += 1;
            }
            if row > 2 && col > 2 && (1..4).all(|x| matrix[row - x][col - x] == target[x - 1]) {
                result += 1;
            }
            if row < height - 3
                && col > 2
                && (1..4).all(|x| matrix[row + x][col - x] == target[x - 1])
            {
                result += 1;
            }
            if row > 2
                && col < width - 3
                && (1..4).all(|x| matrix[row - x][col + x] == target[x - 1])
            {
                result += 1;
            }
            if row < height - 3
                && col < width - 3
                && (1..4).all(|x| matrix[row + x][col + x] == target[x - 1])
            {
                result += 1;
            }
        }
    }
    result
}

fn count_x_mas(lines: &[String]) -> usize {
    let matrix: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let height = matrix.len();
    let width = matrix[0].len();
    let mut result = 0;

    for row in 1..height - 1 {
        for col in 1..width - 1 {
            if matrix[row][col] != 'A' {
                continue;
            }
            if (matrix[row - 1][col - 1] == 'M' && matrix[row + 1][col + 1] == 'S'
                || matrix[row - 1][col - 1] == 'S' && matrix[row + 1][col + 1] == 'M')
                && (matrix[row - 1][col + 1] == 'M' && matrix[row + 1][col - 1] == 'S'
                    || matrix[row - 1][col + 1] == 'S' && matrix[row + 1][col - 1] == 'M')
            {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        assert_eq!(count_xmas(&example), 18);
    }

    #[test]
    fn part2_example() {
        let example = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];

        assert_eq!(count_x_mas(&example), 9);
    }
}
