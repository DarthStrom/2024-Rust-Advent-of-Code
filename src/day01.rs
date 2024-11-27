use crate::input;

pub fn run() {
    let lines = input::get_lines("day01");

    println!("Part1: {:?}", get_solution(&lines));
}

pub fn get_solution(_lines: &[String]) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = vec!["1234".to_string(), "2345".to_string()];

        assert_eq!(get_solution(&example), 42);
    }
}
