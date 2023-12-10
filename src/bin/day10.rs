use itertools::Itertools;

use aoc_2023_rust::aoc;
use Direction::{Left, Right};

use crate::Direction::{Down, Up};

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(10).unwrap();
    let map = Map::parse(&input);
    println!("part1: {}", part1(&map));
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
        cur = next.into_iter().filter(|pos| !visited[pos.x][pos.y]).collect_vec();
        mv_cnt += 1;
    }
    mv_cnt - 1
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

struct Map {
    m: Vec<Vec<char>>, // m[y][x]
}

impl Map {
    fn parse(input: &str) -> Self {
        Self {
            m: input.lines().map(|line| line.chars().collect_vec()).collect_vec()
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.m[0].len(), self.m.len())
    }

    fn get(&self, pos: Pos) -> char {
        self.m[pos.y][pos.x]
    }

    fn starting_pos(&self) -> Pos {
        self.m.iter().enumerate().find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, &c)| {
                if c == 'S' {
                    Some(Pos { x, y })
                } else {
                    None
                }
            })
        }).unwrap()
    }

    fn connects_to(&self, pos: Pos) -> Vec<Pos> {
        self.get(pos).connects().into_iter().filter_map(|d| self.nav(pos, d)).collect_vec()
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
        pos.filter(|&pos| self.get(pos).accepts(direction.opposite()))
    }
}

trait Pipe {
    fn connects(&self) -> Vec<Direction>;
    fn accepts(&self, direction: Direction) -> bool;
}

impl Pipe for char {
    fn connects(&self) -> Vec<Direction> {
        match self {
            '|' => vec![Up, Down],
            '-' => vec![Left, Right],
            'L' => vec![Up, Right],
            'J' => vec![Up, Left],
            '7' => vec![Left, Down],
            'F' => vec![Down, Right],
            'S' => vec![Up, Down, Left, Right],
            _ => vec![],
        }
    }

    fn accepts(&self, direction: Direction) -> bool {
        if *self == 'S' {
            false
        } else {
            self.connects().contains(&direction)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part1(&Map::parse(r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#)), 8);
        assert_eq!(part1(&Map::parse(r#".....
.S-7.
.|.|.
.L-J.
....."#)), 4);
    }
}
