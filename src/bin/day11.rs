use std::ops::Index;

use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(11).unwrap();
    println!("part1: {}", solve(&Map::parse(&input, 1)));
    println!("part2: {}", solve(&Map::parse(&input, 999_999)));
}

fn solve(map: &Map) -> usize {
    map.grid.galaxies().into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            map.distance(map.translate(a), map.translate(b))
        })
        .sum()
}

struct Grid(Vec<Vec<char>>); // [row][col]

impl Grid {
    fn parse(input: &str) -> Self {
        Self(input.lines().map(|line| line.chars().collect_vec()).collect_vec())
    }

    fn size(&self) -> (usize, usize) { // x, y
        (self.0[0].len(), self.0.len())
    }

    fn remap_rows(&self, expand_factor: usize) -> Vec<usize> {
        let mut i = 0;
        let mut remap = Vec::with_capacity(self.0.len());
        for row in self.0.iter() {
            remap.push(i);
            if row.iter().all(|&c| c == '.') {
                i += expand_factor;
            }
            i += 1;
        }
        remap
    }

    fn remap_cols(&self, expand_factor: usize) -> Vec<usize> {
        let mut i = 0;
        let mut remap = Vec::with_capacity(self.0[0].len());
        for col in 0..self.0[0].len() {
            remap.push(i);
            if self.0.iter().all(|row| row[col] == '.') {
                i += expand_factor;
            }
            i += 1;
        }
        remap
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                if self[(col, row)] != '.' {
                    galaxies.push((col, row));
                }
            }
        }
        galaxies
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.0[pos.1][pos.0]
    }
}

struct Map {
    grid: Grid,
    x_remap: Vec<usize>,
    y_remap: Vec<usize>,
}

impl Map {
    fn parse(input: &str, expand_factor: usize) -> Self {
        let grid = Grid::parse(input);
        let x_remap = grid.remap_cols(expand_factor);
        let y_remap = grid.remap_rows(expand_factor);
        Self { grid, x_remap, y_remap }
    }

    fn translate(&self, pos: (usize, usize)) -> (usize, usize) {
        (self.x_remap[pos.0], self.y_remap[pos.1])
    }

    fn distance(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        let (x1, y1) = a;
        let (x2, y2) = b;
        ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
    }
}
