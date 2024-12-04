use regex::Regex;

use crate::day03::{Command, Input, Modifier, Operator};

const EXAMPLE_INPUT: &str = include_str!("../../input/03/example.txt");
const INPUT: &str = include_str!("../../input/03/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    let pattern = r"(?P<operator>don't|mul|do)\(((?P<x>\d{1,3})?,(?P<y>\d{1,3}))?\)";
    let re = Regex::new(pattern).unwrap();

    let mut commands = Vec::new();
    let mut is_on = true;
    for captures in re.captures_iter(input) {
        match captures
            .name("operator")
            .expect("Operator missing")
            .as_str()
        {
            "do" => {
                is_on = true;
                continue;
            }
            "don't" => {
                is_on = false;
                continue;
            }
            _ => (),
        };
        let x: u32 = captures
            .name("x")
            .expect("X missing")
            .as_str()
            .parse()
            .expect("X cannot be parsed as an integer");
        let y: u32 = captures
            .name("y")
            .expect("Y missing")
            .as_str()
            .parse()
            .expect("Y cannot be parsed as an integer");
        commands.push(Command {
            operator: Operator::Mul,
            modifier: if is_on { Modifier::On } else { Modifier::Off },
            x,
            y,
        })
    }

    commands
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();

        let first = input.first().unwrap();
        assert_eq!(first.operator, Operator::Mul);
        assert_eq!(first.x, 345);
        assert_eq!(first.y, 766);

        let last = input.last().unwrap();
        assert_eq!(last.operator, Operator::Mul);
        assert_eq!(last.x, 988);
        assert_eq!(last.y, 195);
    }
}
