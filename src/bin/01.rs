use std::collections::HashMap;

advent_of_code::solution!(1);

fn convert_input_to_groups(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut group1: Vec<u64> = Vec::new();
    let mut group2: Vec<u64> = Vec::new();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"));
        group1.push(nums.next().unwrap());
        group2.push(nums.next().unwrap());
    }

    (group1, group2)
}

pub fn part_one(input: &str) -> Option<u64> {
    let groups = convert_input_to_groups(input);
    let mut group1 = groups.0;
    let mut group2 = groups.1;

    // Sort group1 and group2 in ascending order
    group1.sort();
    group2.sort();

    let total_distance: u64 = group1
        .iter()
        .zip(group2.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let groups = convert_input_to_groups(input);
    let group1 = groups.0;
    let group2 = groups.1;

    let mut cache: HashMap<u64, u64> = HashMap::new();
    let similarity_score: u64 = group1
        .iter()
        .map(|&a| {
            let count = cache
                .entry(a)
                .or_insert_with(|| group2.iter().filter(|&&x| x == a).count() as u64);
            *count * a
        })
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
