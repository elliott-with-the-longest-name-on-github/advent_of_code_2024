use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut safe_reports: i32 = 0;

    for report in input {
        if report_is_safe(report) {
            safe_reports += 1;
        }
    }

    safe_reports.into()
}

enum Direction {
    Ascending,
    Descending,
}

fn report_is_safe(report: &Vec<i32>) -> bool {
    let mut report_iter = report.iter();
    let mut prev_val = report_iter.next().expect("Empty report detected");
    let mut direction: Option<Direction> = None;
    for val in report_iter {
        let diff = prev_val - val;
        if (diff == 0) {
            return false;
        }
        let abs = diff.abs();
        if (abs > 3) {
            return false;
        }
        match direction {
            Some(ref d) => match d {
                Direction::Ascending => {
                    if diff > 0 {
                        return false;
                    };
                }
                Direction::Descending => {
                    if diff < 0 {
                        return false;
                    };
                }
            },
            None => {
                direction = match diff {
                    n if n < 0 => Some(Direction::Ascending),
                    n if n > 0 => Some(Direction::Descending),
                    _ => unreachable!("There should be a guard clause explicitly checking for 0"),
                };
            }
        };
        prev_val = val;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_report_is_safe_no_diff() {
        let report = vec![1, 1, 2, 3, 4];
        let safe = report_is_safe(&report);
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_ascending_too_big() {
        let report = vec![1, 2, 3, 4, 9];
        let safe = report_is_safe(&report);
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_descending_too_small() {
        let report = vec![9, 8, 7, 2, 1];
        let safe = report_is_safe(&report);
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_ascending_direction_changed() {
        let report = vec![1, 2, 3, 2, 1];
        let safe = report_is_safe(&report);
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_descending_direction_changed() {
        let report = vec![3, 2, 1, 2, 3];
        let safe = report_is_safe(&report);
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_ascending_ok() {
        let report = vec![1, 2, 3, 4, 5];
        let safe = report_is_safe(&report);
        assert_eq!(safe, true);
    }

    #[test]
    fn check_report_is_safe_descending_ok() {
        let report = vec![5, 4, 3, 2, 1];
        let safe = report_is_safe(&report);
        assert_eq!(safe, true);
    }
}
