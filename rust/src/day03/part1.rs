use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input.iter().map(|cmd| cmd.x * cmd.y).sum::<u32>().into()
}
