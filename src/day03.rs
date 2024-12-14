use crate::input;
use regex::Regex;

pub fn run() {
    let lines = input::get_lines("day03");

    println!("Part1: {:?}", sum_of_mul(&lines.join("")));
    println!("Part2: {:?}", sum_of_enabled_mul(&lines.join("")));
}

pub fn sum_of_mul(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
    }
    sum
}

pub fn sum_of_enabled_mul(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)(?:n't)?\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for (s, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        match s {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap() * enabled as i32,
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];

        assert_eq!(sum_of_mul(&example.join("")), 161);
    }

    #[test]
    fn multiline_example() {
        let example = vec![
            "^<when()]'>mul(721,815)who()&[how()mul(990,467)select()<".to_string(),
            "*,/~):^[mul(802,939~;!:@mul(656,831)}]?~mul(92,389)/".to_string(),
        ];

        assert_eq!(sum_of_mul(&example.join("")), 1_630_869);
    }

    #[test]
    fn part2_example() {
        let example = vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ];

        assert_eq!(sum_of_enabled_mul(&example.join("")), 48);
    }
}
