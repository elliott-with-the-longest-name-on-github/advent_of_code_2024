use crate::day03::{Input, Modifier, Output};

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|cmd| {
            if let Modifier::Off = cmd.modifier {
                return None;
            }
            Some(cmd.x * cmd.y)
        })
        .sum::<u32>()
        .into()
}
