use crate::day05::{Input, PrintConfig};
use std::collections::{HashMap, HashSet};

const EXAMPLE_INPUT: &str = include_str!("../../input/05/example.txt");
const INPUT: &str = include_str!("../../input/05/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    let (unparsed_rules, unparsed_updates) = input
        .split_once("\n\n")
        .expect("Bad file format while splitting rules and updates");
    let rules = parse_rules(&unparsed_rules);
    let updates = parse_updates(&unparsed_updates);

    PrintConfig { rules, updates }
}

fn parse_rules(unparsed_rules: &str) -> HashMap<u32, HashSet<u32>> {
    unparsed_rules
        .lines()
        .map(|line| {
            let (first, second) = line
                .split_once('|')
                .expect(&format!("Failed to split rule on pipe: {line}"));
            let first = first
                .parse::<u32>()
                .expect(&format!("Failed to parse first page ID in rule: {first}"));
            let second = second
                .parse::<u32>()
                .expect(&format!("Failed to parse second page ID in rule: {second}"));
            (first, second)
        })
        .fold(HashMap::new(), |mut map, (before, after)| {
            map.entry(before).or_insert_with(HashSet::new).insert(after);
            map
        })
}

fn parse_updates(unparsed_updates: &str) -> Vec<Vec<u32>> {
    unparsed_updates
        .lines()
        .map(|rule| {
            rule.split(',')
                .map(|page_id| {
                    page_id
                        .parse::<u32>()
                        .expect(&format!("Could not parse page ID as u32: {page_id}"))
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        let after_89 = input.rules.get(&89).unwrap();
        println!("{:?}", after_89);

        assert_eq!(
            after_89,
            &vec![
                39, 86, 12, 91, 37, 82, 29, 81, 73, 65, 53, 98, 13, 19, 42, 47, 51, 36, 44, 22, 55,
                74, 96, 18
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );

        let first_update_line = input.updates.first().unwrap();
        assert_eq!(
            first_update_line,
            &vec![
                49, 58, 38, 28, 69, 66, 48, 78, 46, 56, 87, 63, 99, 75, 27, 96, 94, 11, 71, 24, 32
            ]
        )
    }
}
