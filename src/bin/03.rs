use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for cap in re.captures_iter(input) {
        let num1: u64 = cap[1].parse().unwrap();
        let num2: u64 = cap[2].parse().unwrap();
        // You can process num1 and num2 as needed
        result += num1 * num2;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_donot = Regex::new(r"don\'t\(\)").unwrap();

    let do_positions: Vec<u64> = re_do.find_iter(input).map(|m| m.start() as u64).collect();
    let donot_positions: Vec<u64> = re_donot
        .find_iter(input)
        .map(|m| m.start() as u64)
        .collect();

    let mut disabled_range: Vec<(u64, u64)> = Vec::new();
    for &donot_pos in &donot_positions {
        match do_positions.iter().find(|&&do_pos| do_pos > donot_pos) {
            Some(&do_pos) => {
                disabled_range.push((donot_pos, do_pos));
            }
            None => {
                disabled_range.push((donot_pos, input.len() as u64));
            }
        }
    }

    for cap in re_mul.captures_iter(input) {
        if let Some(m) = cap.get(0) {
            let start = m.start() as u64;
            if !disabled_range
                .iter()
                .any(|&(a, b)| start >= a && start <= b)
            {
                let num1: u64 = cap[1].parse().unwrap();
                let num2: u64 = cap[2].parse().unwrap();
                // You can process num1 and num2 as needed
                result += num1 * num2;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
