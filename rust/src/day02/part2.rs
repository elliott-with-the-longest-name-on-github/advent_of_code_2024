use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut safe_reports: i32 = 0;

    for report in input {
        // I am 99% sure I got this one right literally by coincidence but tbh I don't have time to investigate further right now
        // if report_is_safe(report.iter()) {
        //     safe_reports += 1;
        //     break;
        // }
        for virtual_report in create_virtual_report_iterator(report) {
            if report_is_safe(virtual_report) {
                safe_reports += 1;
                break;
            }
        }
    }

    safe_reports.into()
}

fn create_virtual_report_iterator(
    report: &Vec<i32>,
) -> Box<dyn Iterator<Item = Box<dyn Iterator<Item = &i32> + '_>> + '_> {
    let mut currently_deleted_key = 0;
    Box::new(
        report
            .iter()
            .enumerate()
            .map(|(virtual_report_number, tv)| {
                Box::new(
                    report
                        .iter()
                        .enumerate()
                        .filter(move |&(i, _)| i != virtual_report_number)
                        .map(|(_, val)| val),
                ) as Box<dyn Iterator<Item = &i32> + '_>
            }),
    )
}

#[derive(PartialEq)]
enum Direction {
    Ascending,
    Descending,
    Flat,
}

fn report_is_safe<'a>(mut report: impl Iterator<Item = &'a i32>) -> bool {
    let mut prev_val = report.next().expect("Empty report detected");
    let mut prev_direction: Option<Direction> = None;
    for val in report {
        let pair_info = parse_pair(prev_val, val);
        let safe = pair_is_safe(&pair_info, &prev_direction);
        if (!safe) {
            return false;
        }
        prev_val = val;
        prev_direction = Some(pair_info.direction);
    }

    true
}

struct PairInfo {
    diff: i32,
    abs_diff: i32,
    direction: Direction,
}

fn parse_pair(left: &i32, right: &i32) -> PairInfo {
    let diff = left - right;
    let abs_diff = diff.abs();
    let direction = match diff {
        n if n < 0 => Direction::Ascending,
        n if n > 0 => Direction::Descending,
        _ => Direction::Flat,
    };

    PairInfo {
        diff,
        abs_diff,
        direction,
    }
}

fn pair_is_safe(pair_info: &PairInfo, prev_direction: &Option<Direction>) -> bool {
    if (pair_info.abs_diff > 3) {
        return false;
    }

    if (pair_info.direction == Direction::Flat) {
        return false;
    }

    if let Some(pd) = prev_direction {
        if (pair_info.direction != *pd) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_report_is_safe_no_diff() {
        let report = vec![1, 1, 1, 3, 4];
        let safe = report_is_safe(report.iter());
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_ascending_too_big() {
        let report = vec![1, 2, 7, 8, 22];
        let safe = report_is_safe(report.iter());
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_descending_too_small() {
        let report = vec![9, 8, 7, 2, 1];
        let safe = report_is_safe(report.iter());
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_ascending_direction_changed() {
        let report = vec![1, 2, 3, 2, 1];
        let safe = report_is_safe(report.iter());
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_descending_direction_changed() {
        let report = vec![3, 2, 1, 2, 3];
        let safe = report_is_safe(report.iter());
        assert_eq!(safe, false);
    }

    #[test]
    fn check_report_is_safe_ascending_ok() {
        let report = vec![1, 2, 3, 4, 5];
        let safe = report_is_safe(report.iter());
        assert_eq!(safe, true);
    }

    #[test]
    fn check_report_is_safe_descending_ok() {
        let report = vec![5, 4, 3, 2, 1];
        let safe = report_is_safe(report.iter());
        assert_eq!(safe, true);
    }
}
