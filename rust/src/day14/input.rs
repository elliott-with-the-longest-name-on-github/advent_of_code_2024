use crate::day14::{Input, Robot};

const EXAMPLE_INPUT: &str = include_str!("../../input/14/example.txt");
const INPUT: &str = include_str!("../../input/14/input.txt");

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
            let (p, v) = line.split_once(' ').unwrap();
            let p = p.replace("p=", "");
            let v = v.replace("v=", "");
            let (p_x, p_y) = p.split_once(',').unwrap();
            let (v_x, v_y) = v.split_once(',').unwrap();
            let p_x = p_x.parse::<i32>().unwrap();
            let p_y = p_y.parse::<i32>().unwrap();
            let v_x = v_x.parse::<i32>().unwrap();
            let v_y = v_y.parse::<i32>().unwrap();
            Robot {
                start_point: (p_x, p_y),
                velocity: (v_x, v_y),
            }
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
        assert_eq!(first.start_point, (26, 88));
        assert_eq!(first.velocity, (80, 92));

        let last = input.last().unwrap();
        assert_eq!(last.start_point, (37, 79));
        assert_eq!(last.velocity, (-41, 70));
    }
}
