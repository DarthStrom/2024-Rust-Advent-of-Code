use crate::input;

pub fn run() {
    let lines = input::get_lines("day01");

    println!("Part1: {:?}", get_sum_of_distances(&lines));
    println!("Part2: {:?}", get_similarity_score(&lines));
}

fn get_sorted_lists(lines: &[String]) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = vec![];
    let mut list2 = vec![];
    lines.iter().for_each(|line| {
        let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
        list1.push(nums.next().unwrap());
        list2.push(nums.next().unwrap());
    });
    list1.sort();
    list2.sort();

    (list1, list2)
}

pub fn get_sum_of_distances(lines: &[String]) -> i32 {
    let (list1, list2) = get_sorted_lists(lines);

    let pairs = list1.iter().zip(list2.iter());
    pairs.map(|(a, b)| (a - b).abs()).sum()
}

pub fn get_similarity_score(lines: &[String]) -> i32 {
    let (list1, list2) = get_sorted_lists(lines);

    list1
        .iter()
        .map(|item1| list2.iter().filter(|item2| item1 == *item2).count() as i32 * item1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];

        assert_eq!(get_sum_of_distances(&example), 11);
    }

    #[test]
    fn part2_example() {
        let example = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];

        assert_eq!(get_similarity_score(&example), 31);
    }
}
