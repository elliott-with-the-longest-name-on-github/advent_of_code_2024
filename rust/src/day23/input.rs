use crate::day23::Input;

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
    Vec::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        let first = input.first();
        assert_eq!(first, None);

        let last = input.last();
        assert_eq!(last, None);
    }
}
