use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let (mut left, mut right) = input.clone();
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|pair| pair.0.abs_diff(*pair.1))
        .sum::<u32>()
        .into()
}
