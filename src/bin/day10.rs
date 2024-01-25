use itertools::Itertools;
use num::abs;

use aoc_2023_rust::aoc;
use Direction::{Left, Right};

use crate::Direction::{Down, Up};

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(10).unwrap();
    let map = Map::parse(&input);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn part1(map: &Map) -> usize {
    let (size_x, size_y) = map.size();
    let mut visited = vec![vec![false; size_x]; size_y];
    let mut mv_cnt = 0usize;
    let mut cur = vec![map.starting_pos()];
    while !cur.is_empty() {
        let mut next = vec![];
        for pos in cur {
            if visited[pos.x][pos.y] {
                continue;
            }
            visited[pos.x][pos.y] = true;
            let connects = map.connects_to(pos);
            next.extend(connects);
        }
        cur = next
            .into_iter()
            .filter(|pos| !visited[pos.x][pos.y])
            .collect_vec();
        mv_cnt += 1;
    }
    mv_cnt - 1
}

fn part2(map: &Map) -> usize {
    let (size_x, size_y) = map.size();
    let mut visited = vec![vec![false; size_x]; size_y];
    let start = map.starting_pos();
    let mut stack = vec![start];
    visited[start.x][start.y] = true;
    let mut boundary = vec![start];
    while let Some(pos) = stack.pop() {
        for pos in map.connects_to(pos) {
            if !visited[pos.x][pos.y] {
                visited[pos.x][pos.y] = true;
                stack.push(pos);
                boundary.push(pos);
            }
        }
    }
    let b = boundary.len();
    // Shoelace: A = sum((y1 + y2) * (x1 - x2)) / 2
    let area = boundary
        .into_iter()
        .circular_tuple_windows()
        .map(|(p1, p2)| (p1.y as i64 + p2.y as i64) * (p1.x as i64 - p2.x as i64))
        .sum::<i64>()
        / 2;
    // Pick's theorem: A = i + b / 2 - 1 => i = A - b / 2 + 1
    abs(area) as usize - b / 2 + 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

struct Map(Vec<Vec<char>>); // [y][x]

impl Map {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        )
    }

    fn size(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    fn get(&self, pos: Pos) -> char {
        self.0[pos.y][pos.x]
    }

    fn starting_pos(&self) -> Pos {
        self.0
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'S' {
                            Some(Pos { x, y })
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap()
    }

    fn connects_to(&self, pos: Pos) -> Vec<Pos> {
        [Up, Down, Left, Right]
            .into_iter()
            .filter(|&d| self.get(pos).connects(d))
            .filter_map(|d| self.nav(pos, d))
            .collect_vec()
    }

    fn nav(&self, pos: Pos, direction: Direction) -> Option<Pos> {
        let (size_x, size_y) = self.size();
        let (x, y) = (pos.x, pos.y);
        let pos = match direction {
            Up if y > 0 => Some(Pos { x, y: y - 1 }),
            Down if y + 1 < size_y => Some(Pos { x, y: y + 1 }),
            Left if x > 0 => Some(Pos { x: x - 1, y }),
            Right if x + 1 < size_x => Some(Pos { x: x + 1, y }),
            _ => None,
        };
        pos.filter(|&pos| self.get(pos).accepts(direction))
    }
}

trait Pipe {
    fn connects(&self, direction: Direction) -> bool;
    fn accepts(&self, direction: Direction) -> bool;
}

impl Pipe for char {
    fn connects(&self, to: Direction) -> bool {
        matches!(
            (self, to),
            ('|', Up | Down)
                | ('-', Left | Right)
                | ('L', Up | Right)
                | ('J', Up | Left)
                | ('7', Left | Down)
                | ('F', Down | Right)
                | ('S', _)
        )
    }

    fn accepts(&self, from: Direction) -> bool {
        if *self == 'S' {
            false
        } else {
            self.connects(from.opposite())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(
            part1(&Map::parse(
                r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#
            )),
            8
        );
        assert_eq!(
            part1(&Map::parse(
                r#".....
.S-7.
.|.|.
.L-J.
....."#
            )),
            4
        );
    }
}
