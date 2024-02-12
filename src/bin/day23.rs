use std::thread;

use grid::Grid;
use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}

fn _main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(23).unwrap();
    let grid = parse_grid(&input);
    println!("part1: {}", solve(&grid, false));
    println!("part2: {}", solve(&grid, true));
}

fn parse_grid(input: &str) -> Grid<char> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
        .into()
}

fn solve(grid: &Grid<char>, ignore_slopes: bool) -> usize {
    dfs(
        grid,
        (0, 1),
        0,
        &mut Grid::init(grid.rows(), grid.cols(), false),
        ignore_slopes,
    )
}

fn dfs(grid: &Grid<char>, pos: Pos, d: usize, vis: &mut Grid<bool>, ignore_slopes: bool) -> usize {
    if pos == (grid.rows() - 1, grid.cols() - 2) {
        return d;
    }
    vis[pos] = true;
    let max = adj(grid, pos, ignore_slopes)
        .map(|adj| {
            if !vis[adj] {
                dfs(grid, adj, d + 1, vis, ignore_slopes)
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0);
    vis[pos] = false;
    max
}

type Pos = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
    fn nav(&self, pos: Pos) -> Option<Pos> {
        let (r, c) = pos;
        let (dr, dc) = self.offset();
        r.checked_add_signed(dr).zip(c.checked_add_signed(dc))
    }

    fn ways(c: char) -> &'static [Direction] {
        match c {
            '^' => &[Direction::Up],
            'v' => &[Direction::Down],
            '<' => &[Direction::Left],
            '>' => &[Direction::Right],
            _ => Self::all_ways(),
        }
    }

    fn all_ways() -> &'static [Direction] {
        &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

fn adj(grid: &Grid<char>, pos: Pos, ignore_slope: bool) -> impl Iterator<Item = Pos> + '_ {
    if ignore_slope {
        Direction::all_ways()
    } else {
        Direction::ways(grid[pos])
    }
    .iter()
    .filter_map(move |d| {
        d.nav(pos)
            .filter(|&(r, c)| grid.get(r, c).is_some_and(|&c| c != '#'))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test() {
        let grid = parse_grid(INPUT);
        assert_eq!(solve(&grid, false), 94);
        assert_eq!(solve(&grid, true), 154);
    }
}
