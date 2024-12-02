use crate::input;

pub fn run() {
    let lines = input::get_lines("day02");

    println!("Part1: {:?}", get_safe_count(&lines, false));
    println!("Part2: {:?}", get_safe_count(&lines, true));
}

fn get_safe_count(lines: &[String], dampen: bool) -> usize {
    lines
        .iter()
        .map(|line| if line_is_safe(line, dampen) { 1 } else { 0 })
        .sum()
}

fn line_is_safe(line: &str, dampen: bool) -> bool {
    let vals = line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let undampened_safe = is_safe(&vals);

    if dampen {
        vals.iter()
            .enumerate()
            .map(|(i, _val)| {
                let mut dampened = vals.clone();
                dampened.remove(i);
                dampened
            })
            .any(|v| is_safe(&v))
    } else {
        undampened_safe
    }
}

fn is_safe(vals: &[i32]) -> bool {
    let mut safe = true;
    let mut increasing: Option<bool> = None;
    let mut peekable = vals.iter().peekable();

    while safe && peekable.peek().is_some() {
        let current = peekable.next();
        let next = peekable.peek();

        if increasing.is_none() {
            increasing = Some(next.is_some_and(|n| n > &current.unwrap()))
        }

        if next.is_some() {
            let n = **next.unwrap();
            let c = *current.unwrap();
            safe = if increasing.unwrap() {
                n > c && n <= c + 3
            } else {
                n < c && n >= c - 3
            };
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];

        assert_eq!(get_safe_count(&example, false), 2);
    }

    #[test]
    fn part2_example() {
        let example = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];

        assert_eq!(get_safe_count(&example, true), 4);
    }
}
