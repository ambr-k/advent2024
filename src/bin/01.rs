use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = BinaryHeap::<u32>::new();
    let mut right = BinaryHeap::<u32>::new();
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .for_each(|pair| {
            left.push(pair[0].parse().unwrap());
            right.push(pair[1].parse().unwrap());
        });
    Some(
        left.into_sorted_vec()
            .into_iter()
            .zip(right.into_sorted_vec().into_iter())
            .map(|(l, r)| l.abs_diff(r))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = Vec::<u32>::new();
    let mut right = HashMap::<u32, u32>::new();
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .for_each(|pair| {
            left.push(pair[0].parse().unwrap());
            let r = pair[1].parse().unwrap();
            right.insert(r, right.get(&r).copied().unwrap_or_default() + 1);
        });
    Some(
        left.into_iter()
            .map(|l| l * right.get(&l).copied().unwrap_or_default())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
