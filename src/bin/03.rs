use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#)
            .unwrap()
            .captures_iter(input)
            .map(|capture| capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut do_mul = true;
    Some(
        Regex::new(r#"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)"#)
            .unwrap()
            .captures_iter(input)
            .map(|capture| match &capture[0] {
                "do()" => {
                    do_mul = true;
                    0
                }
                "don't()" => {
                    do_mul = false;
                    0
                }
                _ => {
                    if do_mul {
                        capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap()
                    } else {
                        0
                    }
                }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
