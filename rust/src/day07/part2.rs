use crate::day07::{Equation, Input, Output};

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .filter_map(|equation| {
            if (is_realistic_equation(equation.goal, equation.numbers[0], &equation.numbers, 1)) {
                return Some(equation.goal);
            }
            None
        })
        .sum::<i64>()
        .into()
}

fn is_realistic_equation(goal: i64, accumulator: i64, numbers: &[i64], index: usize) -> bool {
    if let Some(next_number) = numbers.get(index) {
        let addition_accumulator = accumulator + next_number;
        let addition_worked = is_realistic_equation(goal, addition_accumulator, numbers, index + 1);
        if addition_worked {
            return true;
        }
        let multiplication_accumulator = accumulator * next_number;
        let multiplication_worked =
            is_realistic_equation(goal, multiplication_accumulator, numbers, index + 1);
        if (multiplication_worked) {
            return true;
        }
        let concatendation_accumulator = (accumulator.to_string() + &next_number.to_string())
            .parse::<i64>()
            .unwrap();
        is_realistic_equation(goal, concatendation_accumulator, numbers, index + 1)
    } else {
        // we're at the end of the chain, return whether or not goal === accumulator
        return goal == accumulator;
    }
}
