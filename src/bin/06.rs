use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn velocity(&self) -> (isize, isize) {
        match &self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn in_bounds(grid: &Vec<Vec<char>>, point: (isize, isize)) -> bool {
    (0 <= point.0 && point.0 < grid.len() as isize)
        && (0 <= point.1 && point.1 < grid[point.0 as usize].len() as isize)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let guard_idx = input.find('^').unwrap();
    let line_len = input.find('\n').unwrap() + 1;
    let mut guard = (guard_idx / line_len, guard_idx % line_len);
    assert_eq!(grid[guard.0][guard.1], '^');
    grid[guard.0][guard.1] = 'X';

    let mut direction = Direction::Up;

    loop {
        let velocity = direction.velocity();
        let next_isize = (guard.0 as isize + velocity.0, guard.1 as isize + velocity.1);
        if !in_bounds(&grid, next_isize) {
            break;
        }

        let next = (next_isize.0 as usize, next_isize.1 as usize);
        if grid[next.0][next.1] == '#' {
            direction = direction.turn_right();
        } else {
            grid[next.0][next.1] = 'X';
            guard = next;
        }
    }

    Some(
        grid.into_iter()
            .map(|row| row.into_iter().filter(|c| *c == 'X').count() as u32)
            .sum(),
    )
}

fn creates_cycle(
    grid: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_direction: Direction,
    obstacle: (usize, usize),
) -> bool {
    let mut visited = HashSet::<((usize, usize), Direction)>::new();
    let mut guard = start_pos;
    let mut direction = start_direction;

    loop {
        if visited.contains(&(guard, direction)) {
            return true;
        }
        visited.insert((guard, direction));
        let velocity = direction.velocity();
        let next_isize = (guard.0 as isize + velocity.0, guard.1 as isize + velocity.1);
        if !in_bounds(grid, next_isize) {
            return false;
        }

        let next = (next_isize.0 as usize, next_isize.1 as usize);
        if grid[next.0][next.1] == '#' || (next == obstacle) {
            direction = direction.turn_right();
        } else {
            guard = next;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let guard_idx = input.find('^').unwrap();
    let line_len = input.find('\n').unwrap() + 1;
    let mut guard = (guard_idx / line_len, guard_idx % line_len);
    assert_eq!(grid[guard.0][guard.1], '^');

    let mut direction = Direction::Up;

    let mut obstacles = HashSet::<(usize, usize)>::new();
    let mut path = HashSet::<(usize, usize)>::new();

    loop {
        path.insert(guard);

        let velocity = direction.velocity();
        let next_isize = (guard.0 as isize + velocity.0, guard.1 as isize + velocity.1);
        if !in_bounds(&grid, next_isize) {
            break;
        }

        let next = (next_isize.0 as usize, next_isize.1 as usize);
        if grid[next.0][next.1] == '#' {
            direction = direction.turn_right();
        } else {
            if !path.contains(&next)
                && !obstacles.contains(&next)
                && creates_cycle(&grid, guard, direction, next)
            {
                obstacles.insert(next);
            }
            guard = next;
        }
    }

    Some(obstacles.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
