use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};

use crate::day05::{Input, Output};

pub fn solve(input: &Input) -> Output {
    input
        .updates
        .iter()
        .filter(|page_id| page_id.is_sorted_by(|a, b| input.rules.get(a).unwrap().contains(b)))
        .map(|page_id| page_id[page_id.len() / 2])
        .sum::<u32>()
        .into()
}
