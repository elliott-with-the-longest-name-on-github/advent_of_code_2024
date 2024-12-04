use crate::day01::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/01/example.txt");
const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in input.split('\n') {
        let mut line_iter = line.split("   ").map(|unparsed_val| {
            unparsed_val
                .parse::<u32>()
                .expect(&format!("Value {unparsed_val} cannot be parsed into a u32"))
        });
        let left_val = line_iter
            .next()
            .expect(&format!("Missing first value on line {line}"));
        let right_val = line_iter
            .next()
            .expect(&format!("Missing second value on line {line}"));
        left.push(left_val);
        right.push(right_val);
    }

    (left, right)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let (left, right) = read_example();

        let first_left = *left.first().unwrap();
        assert_eq!(first_left, 3);

        let first_right = *right.first().unwrap();
        assert_eq!(first_right, 4);

        let second_left = *left.last().unwrap();
        assert_eq!(second_left, 3);

        let second_right = *right.last().unwrap();
        assert_eq!(second_right, 3);
    }
}
