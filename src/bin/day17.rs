use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::RangeInclusive;

use grid::Grid;
use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(17).unwrap();
    let grid = parse_map(&input);
    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

type Map = Grid<u8>;

fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec()
        .into()
}

fn part1(grid: &Map) -> u32 {
    solve(grid, 1..=3)
}

fn part2(grid: &Map) -> u32 {
    solve(grid, 4..=10)
}

fn solve(grid: &Map, steps: RangeInclusive<usize>) -> u32 {
    let mut visited = Grid::init(grid.rows(), grid.cols(), [false; 4]);
    let mut next = BinaryHeap::new();
    next.push((Reverse(0u32), (0usize, 0usize), Direction::Right));
    next.push((Reverse(0u32), (0usize, 0usize), Direction::Down));
    while let Some((Reverse(cost), pos, dir)) = next.pop() {
        if visited[pos][dir as usize] {
            continue;
        }
        visited[pos][dir as usize] = true;
        if pos == (grid.rows() - 1, grid.cols() - 1) {
            return cost;
        }
        for d in [dir.rotate_left(), dir.rotate_right()] {
            let mut pos = pos;
            let mut cost = cost;
            for step in 1..=*steps.end() {
                pos = match nav(grid, pos, d) {
                    None => break,
                    Some(p) => p,
                };
                cost += grid[pos] as u32;
                if step >= *steps.start() {
                    next.push((Reverse(cost), pos, d));
                }
            }
        }
    }
    unreachable!()
}

type Pos = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
}

fn nav<T>(grid: &Grid<T>, pos: Pos, dir: Direction) -> Option<Pos> {
    let (y, x) = pos;
    match dir {
        Direction::Up => {
            if y == 0 {
                None
            } else {
                Some((y - 1, x))
            }
        }
        Direction::Down => {
            if y == grid.rows() - 1 {
                None
            } else {
                Some((y + 1, x))
            }
        }
        Direction::Left => {
            if x == 0 {
                None
            } else {
                Some((y, x - 1))
            }
        }
        Direction::Right => {
            if x == grid.cols() - 1 {
                None
            } else {
                Some((y, x + 1))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn p1() {
        let grid = parse_map(INPUT);
        assert_eq!(part1(&grid), 102);
    }
}
