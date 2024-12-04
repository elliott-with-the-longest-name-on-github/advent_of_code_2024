use crate::day01::{Input, Output};
use std::collections::HashMap;

pub fn solve(input: &Input) -> Output {
    let (mut left, mut right) = input.clone();
    let frequency_map = right.iter().fold(HashMap::new(), |mut map, &value| {
        *map.entry(value).or_insert(0) += 1;
        map
    });
    left.iter()
        .map(|x| x * frequency_map.get(x).unwrap_or(&0))
        .sum::<u32>()
        .into()
}
