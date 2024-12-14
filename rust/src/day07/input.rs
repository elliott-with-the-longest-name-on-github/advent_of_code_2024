use crate::day07::{Equation, Input};

const EXAMPLE_INPUT: &str = include_str!("../../input/07/example.txt");
const INPUT: &str = include_str!("../../input/07/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (unparsed_goal, unparsed_numbers) = line.split_once(": ").unwrap();
            let goal = unparsed_goal.parse::<i64>().unwrap();
            let numbers = unparsed_numbers
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            Equation { goal, numbers }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        let first = input.first().unwrap();
        assert_eq!(first.goal, 426048);
        assert_eq!(first.numbers, vec![425, 608, 69, 88, 1, 282]);

        let last = input.last().unwrap();
        assert_eq!(last.goal, 1680716);
        assert_eq!(last.numbers, vec![236, 37, 35, 71, 3]);
    }
}
