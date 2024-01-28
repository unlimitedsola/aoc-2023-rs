use grid::Grid;
use itertools::{chain, Itertools};

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(16).unwrap();
    let grid = parse_grid(&input);
    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

fn parse_grid(input: &str) -> Grid<char> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec()
        .into()
}

fn part1(grid: &Grid<char>) -> usize {
    solve(grid, (0, 0), Direction::Right)
}

fn part2(grid: &Grid<char>) -> usize {
    chain!(
        (0..grid.rows()).map(|y| ((y, 0), Direction::Right)),
        (0..grid.rows()).map(|y| ((y, grid.cols() - 1), Direction::Left)),
        (0..grid.cols()).map(|x| ((0, x), Direction::Down)),
        (0..grid.cols()).map(|x| ((grid.rows() - 1, x), Direction::Up)),
    )
    .map(|(pos, dir)| solve(grid, pos, dir))
    .max()
    .unwrap()
}

type Pos = (usize, usize);

#[derive(Debug, Copy, Clone)]
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
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

fn solve(grid: &Grid<char>, pos: Pos, dir: Direction) -> usize {
    let mut visited = Grid::init(grid.rows(), grid.cols(), [false; 4]);
    let mut beams = vec![(pos, dir)];
    while let Some((pos, dir)) = beams.pop() {
        if visited[pos][dir as usize] {
            continue;
        }
        visited[pos][dir as usize] = true;
        for next in tick(grid, pos, dir) {
            beams.push(next);
        }
    }
    visited.iter().filter(|&a| a.iter().any(|&b| b)).count()
}

fn tick(grid: &Grid<char>, pos: Pos, dir: Direction) -> Vec<(Pos, Direction)> {
    let c = grid[pos];
    match (c, dir) {
        ('|', Direction::Left | Direction::Right) => [Direction::Up, Direction::Down]
            .into_iter()
            .filter_map(|d| nav(grid, pos, d))
            .collect_vec(),
        ('-', Direction::Up | Direction::Down) => [Direction::Left, Direction::Right]
            .into_iter()
            .filter_map(|d| nav(grid, pos, d))
            .collect_vec(),
        ('/', _) => nav(grid, pos, dir.rotate_right()).into_iter().collect_vec(),
        ('\\', _) => nav(grid, pos, dir.rotate_left()).into_iter().collect_vec(),
        _ => nav(grid, pos, dir).into_iter().collect_vec(),
    }
}

fn nav(grid: &Grid<char>, pos: Pos, dir: Direction) -> Option<(Pos, Direction)> {
    let (y, x) = pos;
    match dir {
        Direction::Up => {
            if y == 0 {
                None
            } else {
                Some(((y - 1, x), dir))
            }
        }
        Direction::Down => {
            if y == grid.rows() - 1 {
                None
            } else {
                Some(((y + 1, x), dir))
            }
        }
        Direction::Left => {
            if x == 0 {
                None
            } else {
                Some(((y, x - 1), dir))
            }
        }
        Direction::Right => {
            if x == grid.cols() - 1 {
                None
            } else {
                Some(((y, x + 1), dir))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn p1() {
        let grid = parse_grid(INPUT);
        assert_eq!(part1(&grid), 46)
    }
}
