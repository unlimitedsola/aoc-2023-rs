use itertools::Itertools;
use num::abs;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(18).unwrap();
    println!("part1: {}", solve(&parse_plan1(&input)));
    println!("part2: {}", solve(&parse_plan2(&input)));
}

fn parse_plan1(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|l| {
            let (d, n, _) = l.split_ascii_whitespace().collect_tuple().unwrap();
            (d.chars().exactly_one().unwrap(), n.parse().unwrap())
        })
        .collect_vec()
}

fn parse_plan2(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|l| {
            let (_, c) = l.split_once('#').unwrap();
            (
                c.chars().nth(5).unwrap(),
                i64::from_str_radix(&c[..5], 16).unwrap(),
            )
        })
        .collect_vec()
}

fn solve(steps: &[(char, i64)]) -> i64 {
    let mut y = 0;
    let mut a = 1;
    let mut b = 0;
    for (d, n) in steps {
        // shoelace: A = sum((y1 + y2) * (x1 - x2)) / 2
        // since we only move on one axis at a time, -dx = (x2 - x1) is either 0 or -n,
        // further simplifies to A = sum(y * -dx)
        let (dy, dx) = dv(*d);
        y += dy * n;
        a += dx * -n * y;
        b += n;
    }
    abs(a) + b / 2
}

fn dv(d: char) -> (i64, i64) {
    match d {
        'R' | '0' => (0, 1),
        'D' | '1' => (1, 0),
        'L' | '2' => (0, -1),
        'U' | '3' => (-1, 0),
        _ => unreachable!("invalid direction: {}", d),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test() {
        assert_eq!(solve(&parse_plan1(INPUT)), 62);
        assert_eq!(solve(&parse_plan2(INPUT)), 952408144115);
    }
}
