use std::str::Lines;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(5).unwrap();

    let mut lines = input.lines();
    let seeds = lines.next().unwrap();
    lines.next();
    let maps = parse_maps(&mut lines);

    let seeds1 = parse_seeds1(seeds);
    part1(seeds1, &maps);
}

#[derive(Debug)]
struct Mapping {
    src: u64,
    dst: u64,
    len: u64,
}

impl Mapping {
    fn apply1(&self, seed: u64) -> Option<u64> {
        if (self.src..self.src + self.len).contains(&seed) {
            Some(self.dst + seed - self.src)
        } else {
            None
        }
    }

    fn parse(line: &str) -> Self {
        let [dst, src, len] =
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>()
                .try_into().unwrap();
        Self { src, dst, len }
    }
}

fn parse_maps(lines: &mut Lines) -> Vec<Vec<Mapping>> {
    let mut maps = vec![];
    while let Some(_) = lines.next() {
        let mappings = lines
            .take_while(|l| !l.is_empty())
            .map(Mapping::parse)
            .collect();
        maps.push(mappings);
    };
    maps
}

fn parse_seeds1(line: &str) -> Vec<u64> {
    line.strip_prefix("seeds: ").unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1<I>(mut seeds: Vec<u64>, maps: &[I]) where I: AsRef<[Mapping]> {
    for map in maps {
        for seed in seeds.iter_mut() {
            for mapping in map.as_ref() {
                if let Some(dst) = mapping.apply1(*seed) {
                    *seed = dst;
                    break;
                }
            }
        }
    }
    seeds.sort();
    println!("part1: {}", seeds.first().unwrap())
}
