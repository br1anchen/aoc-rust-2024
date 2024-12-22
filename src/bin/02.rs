advent_of_code::solution!(2);

fn is_safe_report(nums: Vec<i64>) -> bool {
    let mut diffs: Vec<i64> = Vec::new();
    for (prev, num) in nums.iter().zip(nums.iter().skip(1)) {
        diffs.push(num - prev);
    }

    let decreasings = diffs.iter().filter(|&&x| x < 0).count();
    let increasings = diffs.iter().filter(|&&x| x > 0).count();
    let valid_diffs = diffs
        .iter()
        .filter(|&&x| x.abs() >= 1 && x.abs() <= 3)
        .count();

    let mismatched_order = decreasings != 0 && increasings != 0;

    !mismatched_order && valid_diffs == diffs.len()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut safe_report_count = 0;

    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect::<Vec<i64>>();

        safe_report_count += match is_safe_report(nums) {
            true => 1,
            false => 0,
        };
    }

    Some(safe_report_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut safe_report_count = 0;

    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect::<Vec<i64>>();

        if is_safe_report(nums.clone()) {
            safe_report_count += 1;
            continue;
        }

        for i in 0..nums.len() {
            let mut nums = nums.clone();
            nums.remove(i);
            if is_safe_report(nums) {
                safe_report_count += 1;
                break;
            }
        }

        // let mut diffs: Vec<i64> = Vec::new();
        // for (prev, num) in nums.iter().zip(nums.iter().skip(1)) {
        //     diffs.push(num - prev);
        // }
        //
        // let decreasings = diffs.iter().filter(|&&x| x < 0).count();
        // let increasings = diffs.iter().filter(|&&x| x > 0).count();
        //
        // let mut single_outline_index = None;
        // if increasings > decreasings && decreasings == 1 {
        //     single_outline_index = diffs.iter().position(|&x| x < 0);
        // }
        //
        // if decreasings > increasings && increasings == 1 {
        //     single_outline_index = diffs.iter().position(|&x| x > 0);
        // }
        //
        // if single_outline_index.is_some() {
        //     let mut nums = nums.clone();
        //     let index_to_remove = if increasings > decreasings {
        //         single_outline_index.unwrap()
        //     } else {
        //         single_outline_index.unwrap() + 1
        //     };
        //     nums.remove(index_to_remove);
        //     if is_safe_report(nums) {
        //         safe_report_count += 1;
        //     }
        // }
        //
        // let invalid_diff_index = diffs.iter().position(|&x| x.abs() < 1 || x.abs() > 3);
        //
        // if invalid_diff_index.is_some() && single_outline_index.is_none() {
        //     let mut nums = nums.clone();
        //     nums.remove(invalid_diff_index.unwrap() + 1);
        //     if is_safe_report(nums) {
        //         safe_report_count += 1;
        //     }
        // }
    }

    Some(safe_report_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
