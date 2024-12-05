use crate::day05::{Input, Output};
use std::cmp::Ordering::*;

pub fn solve(input: &Input) -> Output {
    let mut updates = input.updates.clone();
    updates
        .iter_mut()
        .filter(|page| !page.is_sorted_by(|a, b| input.rules[a].contains(b)))
        .map(|mut page| {
            page.sort_by(|a, b| {
                if input.rules[a].contains(b) {
                    Less
                } else {
                    Greater
                }
            });
            page[page.len() / 2]
        })
        .sum::<u32>()
        .into()
}
