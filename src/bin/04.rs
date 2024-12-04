use itertools::Itertools;

advent_of_code::solution!(4);

fn is_match(
    grid: &Vec<Vec<char>>,
    (row, col): (isize, isize),
    (d_row, d_col): (isize, isize),
) -> bool {
    ((0 <= row + 3 * d_row) && (row + 3 * d_row < grid.len() as isize))
        && grid[(row + 0 * d_row) as usize].get((col + 0 * d_col) as usize) == Some(&'X')
        && grid[(row + 1 * d_row) as usize].get((col + 1 * d_col) as usize) == Some(&'M')
        && grid[(row + 2 * d_row) as usize].get((col + 2 * d_col) as usize) == Some(&'A')
        && grid[(row + 3 * d_row) as usize].get((col + 3 * d_col) as usize) == Some(&'S')
}

fn n_matches(grid: &Vec<Vec<char>>, (row, col): (usize, usize)) -> u32 {
    const DIRS: [isize; 3] = [-1, 0, 1];
    DIRS.into_iter()
        .cartesian_product(DIRS.into_iter())
        .filter(|d| is_match(grid, (row as isize, col as isize), *d))
        .count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    Some(
        (0..grid.len())
            .map(|row| {
                (0..grid[row].len())
                    .map(|col| n_matches(&grid, (row, col)))
                    .sum::<u32>()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    Some(
        (1..(grid.len() - 1))
            .map(|row| {
                (1..(grid[row].len() - 1))
                    .filter(|col| {
                        (grid[row][*col] == 'A')
                            && ((grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
                                || (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M'))
                            && ((grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S')
                                || (grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M'))
                    })
                    .count() as u32
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
