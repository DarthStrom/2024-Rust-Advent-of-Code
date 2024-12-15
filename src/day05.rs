use std::collections::HashSet;

use crate::input;

pub fn run() {
    let input = input::get_contents("day05");

    println!("Part1: {:?}", middle_page_sum(&input));
}

fn middle_page_sum(input: &str) -> u32 {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let valid_updates = find_valid_updates(
        &input::get_lines_str(rules_str),
        &input::get_lines_str(updates_str),
    );

    valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn build_rules(rule_strs: &[String]) -> HashSet<(u32, u32)> {
    let rules: HashSet<(u32, u32)> = rule_strs
        .iter()
        .map(|rule| (rule[0..2].parse().unwrap(), rule[3..].parse().unwrap()))
        .collect();
    rules
}

fn find_valid_updates(rule_strs: &[String], update_strs: &[String]) -> Vec<Vec<u32>> {
    let rules = build_rules(rule_strs);

    let updates = parse_updates(update_strs);

    updates
        .into_iter()
        .filter(|update| update.is_sorted_by(|a, b| rules.contains(&(*a, *b))))
        .collect()
}

fn parse_updates(update_strs: &[String]) -> Vec<Vec<u32>> {
    update_strs
        .iter()
        .map(|update_str| {
            update_str
                .split(",")
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_rules() -> Vec<String> {
        vec![
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
        ]
    }

    #[test]
    fn building_rules() {
        assert_eq!(
            build_rules(&get_example_rules()),
            HashSet::from([
                (29, 13),
                (47, 13),
                (47, 29),
                (47, 53),
                (47, 61),
                (53, 13),
                (53, 29),
                (61, 13),
                (61, 29),
                (61, 53),
                (75, 13),
                (75, 29),
                (75, 47),
                (75, 53),
                (75, 61),
                (97, 13),
                (97, 29),
                (97, 47),
                (97, 53),
                (97, 61),
                (97, 75),
            ])
        );
    }

    #[test]
    fn finding_valid_updates() {
        let rules = get_example_rules();

        let updates = vec![
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ];

        assert_eq!(find_valid_updates(&rules, &updates), vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ]);
    }
}