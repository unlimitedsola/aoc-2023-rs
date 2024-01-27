use std::collections::HashMap;

use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(14).unwrap();
    let grid = parse_grid(&input);
    println!("part1: {}", part1(grid.clone()));
    println!("part2: {}", part2(grid.clone()));
}

type Grid = grid::Grid<char>;

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec()
        .into()
}

fn part1(mut grid: Grid) -> usize {
    tick(&mut grid);
    weight(&grid)
}

fn part2(mut grid: Grid) -> usize {
    let mut seen = HashMap::new();
    let count = 1_000_000_000;
    for i in 0..count {
        if let Some(j) = seen.get(&grid) {
            let cycle = i - j;
            let remaining = count - i;
            let remaining = remaining % cycle;
            for _ in 0..remaining {
                tick4(&mut grid);
            }
            break;
        }
        seen.insert(grid.clone(), i);
        tick4(&mut grid);
    }
    weight(&grid)
}

fn tick4(grid: &mut Grid) {
    for _ in 0..4 {
        tick(grid);
        grid.rotate_right();
    }
}

fn tick(grid: &mut Grid) {
    let mut heights = vec![0; grid.cols()];
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let c = grid[(y, x)];
            match c {
                'O' => {
                    grid[(y, x)] = '.';
                    grid[(heights[x], x)] = 'O';
                    heights[x] += 1;
                }
                '#' => {
                    heights[x] = y + 1;
                }
                _ => {}
            }
        }
    }
}

fn weight(grid: &Grid) -> usize {
    grid.indexed_iter()
        .filter(|(_, &c)| c == 'O')
        .map(|((y, _), _)| grid.rows() - y)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn p1() {
        assert_eq!(part1(parse_grid(INPUT)), 136)
    }

    #[test]
    fn p2() {
        assert_eq!(part2(parse_grid(INPUT)), 64)
    }
}
