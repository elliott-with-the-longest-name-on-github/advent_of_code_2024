use crate::day02::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/02/example.txt");
const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|val| {
                    val.parse::<i32>()
                        .expect(&format!("Value {val} cannot be parsed as an integer"))
                })
                .collect()
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
        assert_eq!(first, &vec![74, 76, 78, 79, 76]);

        let last = input.last().unwrap();
        assert_eq!(last, &vec![59, 58, 55, 53, 52]);
    }
}
