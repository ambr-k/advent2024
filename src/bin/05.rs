use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_str, jobs_str) = input.split_once("\n\n").unwrap();
    let rules = rules_str
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<HashSet<_>>();
    Some(
        jobs_str
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .filter(|job| {
                let mut prevs = HashSet::<u32>::new();
                job.into_iter().all(|current| {
                    prevs.insert(*current);
                    !prevs.iter().any(|prev| rules.contains(&(*current, *prev)))
                })
            })
            .map(|job| job[job.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_str, jobs_str) = input.split_once("\n\n").unwrap();
    let rules = rules_str
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<HashSet<_>>();
    Some(
        jobs_str
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .filter(|job| {
                let mut prevs = HashSet::<u32>::new();
                job.into_iter().any(|current| {
                    prevs.insert(*current);
                    prevs.iter().any(|prev| rules.contains(&(*current, *prev)))
                })
            })
            .map(|job| {
                let mut ordered = job.clone();
                let mid_idx = ordered.len() / 2;
                let mut idx = 0;
                while idx < ordered.len() {
                    let reversed_pair = if idx < ordered.len() / 2 {
                        (ordered[mid_idx], ordered[idx])
                    } else {
                        (ordered[idx], ordered[mid_idx])
                    };
                    if rules.contains(&reversed_pair) {
                        ordered.swap(idx, mid_idx);
                        idx = 0;
                    } else {
                        idx += 1;
                    }
                }
                ordered[mid_idx]
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
