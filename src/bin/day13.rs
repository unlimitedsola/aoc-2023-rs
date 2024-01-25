use std::iter::zip;

use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(13).unwrap();
    part1(&input);
    part2(&input);
}

type Grid = grid::Grid<char>;

fn part1(input: &str) {
    let grids = parse(input);
    let sum: usize = grids.into_iter().map(|g| solve(g, 0)).sum();
    println!("part1: {}", sum);
}

fn part2(input: &str) {
    let grids = parse(input);
    let sum: usize = grids.into_iter().map(|g| solve(g, 1)).sum();
    println!("part2: {}", sum);
}

fn solve(mut grid: Grid, tolerance: usize) -> usize {
    (1..grid.rows())
        .find(|&i| mirror_diff(&grid, i) == tolerance)
        .map(|i| i * 100)
        .or_else(|| {
            grid.transpose();
            (1..grid.rows()).find(|&i| mirror_diff(&grid, i) == tolerance)
        })
        .unwrap()
}

fn mirror_diff(grid: &Grid, i: usize) -> usize {
    if i == 0 {
        unreachable!()
    }
    zip((0..i).rev(), i..grid.rows())
        .map(|(l, r)| {
            grid.iter_row(l)
                .zip(grid.iter_row(r))
                .filter(|(l, r)| l != r)
                .count()
        })
        .sum()
}

fn parse(input: &str) -> Vec<Grid> {
    let mut grids = vec![];
    let mut lines = input.lines();
    loop {
        let grid = (&mut lines)
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        if grid.is_empty() {
            break;
        }
        grids.push(grid.into());
    }
    grids
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn p1() {
        let grids = parse(INPUT);
        assert_eq!(grids.len(), 2);
        assert_eq!(solve(grids[0].clone(), 0), 5);
        assert_eq!(solve(grids[1].clone(), 0), 400);
    }

    #[test]
    fn p2() {
        let grids = parse(INPUT);
        assert_eq!(solve(grids[0].clone(), 1), 300);
        assert_eq!(solve(grids[1].clone(), 1), 100);
    }
}
