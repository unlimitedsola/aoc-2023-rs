use std::collections::HashMap;
use std::str::Lines;

use itertools::Itertools;
use num::integer::lcm;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(8).unwrap();
    let mut lines = input.lines();
    let moves = lines.next().unwrap().chars().collect_vec();
    lines.next();
    let locs = Loc::parse(lines);
    part1(&moves, &locs);
    part2(&moves, &locs);
}

#[derive(Debug)]
struct Loc(String, String);

impl Loc {
    fn parse(lines: Lines) -> HashMap<String, Loc> {
        lines.map(|line| (
            line[0..3].to_string(),
            Loc(line[7..10].to_owned(), line[12..15].to_owned())
        )).collect()
    }

    fn nav(&self, d: char) -> &str {
        match d {
            'L' => &self.0,
            'R' => &self.1,
            _ => unreachable!("invalid direction: {}", d)
        }
    }
}

fn part1(moves: &[char], locs: &HashMap<String, Loc>) {
    let mut mv_cnt = 0usize;
    let mut cur = locs.get("AAA").unwrap();
    'main: loop {
        for d in moves {
            mv_cnt += 1;
            let dst = cur.nav(*d);
            if dst == "ZZZ" {
                break 'main;
            }
            cur = locs.get(dst).unwrap();
        }
    }
    println!("part1: {}", mv_cnt)
}

fn part2(moves: &[char], locs: &HashMap<String, Loc>) {
    let src = locs.keys().filter(|k| k.ends_with('A')).collect_vec();
    let cnt = src.iter().map(|&s| part2_len(locs.get(s).unwrap(), moves, locs)).collect_vec();
    let ans = cnt.into_iter().reduce(lcm).unwrap();
    println!("part2: {}", ans)
}

fn part2_len<'a>(mut cur: &'a Loc, moves: &[char], locs: &'a HashMap<String, Loc>) -> usize {
    let mut mv_cnt = 0usize;
    'main: loop {
        for d in moves {
            mv_cnt += 1;
            let dst = cur.nav(*d);
            if dst.ends_with('Z') {
                break 'main;
            }
            cur = locs.get(dst).unwrap();
        }
    };
    mv_cnt
}
