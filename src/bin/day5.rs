use std::cmp::{max, min};
use std::str::Lines;

use itertools::Itertools;

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

    let seeds2 = parse_seeds2(seeds);
    part2(seeds2, &maps);
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

    fn apply2(&self, seed: &Seed) -> Option<(Seed, Vec<Seed>)> {
        if !(seed.i..seed.i + seed.len).contains(&self.src)
            && !(self.src..self.src + self.len).contains(&seed.i)
        {
            return None;
        }
        let start = max(self.src, seed.i);
        let end = min(self.src + self.len, seed.i + seed.len);
        let len = end - start;
        let mapped = Seed {
            i: self.dst + start - self.src,
            len,
        };

        let mut rem = vec![];
        if seed.i < self.src {
            // head
            rem.push(Seed {
                i: seed.i,
                len: self.src - seed.i,
            });
        }
        if seed.i + seed.len > self.src + self.len {
            // tail
            rem.push(Seed {
                i: end,
                len: seed.i + seed.len - end,
            });
        }
        Some((mapped, rem))
    }

    fn parse(line: &str) -> Self {
        let [dst, src, len] = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();
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
    }
    maps
}

fn parse_seeds1(line: &str) -> Vec<u64> {
    line.strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1<I>(mut seeds: Vec<u64>, maps: &[I])
where
    I: AsRef<[Mapping]>,
{
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Seed {
    i: u64,
    len: u64,
}

fn parse_seeds2(line: &str) -> Vec<Seed> {
    line.strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .tuples()
        .map(|(i, len)| Seed { i, len })
        .collect()
}

fn part2<I>(mut seeds: Vec<Seed>, maps: &[I])
where
    I: AsRef<[Mapping]>,
{
    for map in maps {
        seeds = do_map(seeds, map.as_ref());
    }
    seeds.sort_by_key(|s| s.i);
    println!("part2: {}", seeds.first().unwrap().i)
}

fn do_map(mut seeds: Vec<Seed>, map: &[Mapping]) -> Vec<Seed> {
    let mut res = vec![];
    'seed: while let Some(seed) = seeds.pop() {
        for mapping in map {
            if let Some((mapped, rem)) = mapping.apply2(&seed) {
                res.push(mapped);
                seeds.extend(rem);
                continue 'seed;
            }
        }
        res.push(seed)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_apply() {
        let m = Mapping {
            src: 10,
            dst: 100,
            len: 10,
        };
        assert_eq!(
            m.apply2(&Seed { i: 5, len: 15 }),
            Some((Seed { i: 100, len: 10 }, vec![Seed { i: 5, len: 5 }]))
        );
        assert_eq!(m.apply2(&Seed { i: 25, len: 15 }), None);
        assert_eq!(
            m.apply2(&Seed { i: 15, len: 15 }),
            Some((Seed { i: 105, len: 5 }, vec![Seed { i: 20, len: 10 }]))
        );
        assert_eq!(
            m.apply2(&Seed { i: 10, len: 10 }),
            Some((Seed { i: 100, len: 10 }, vec![]))
        );
        assert_eq!(
            m.apply2(&Seed { i: 12, len: 3 }),
            Some((Seed { i: 102, len: 3 }, vec![]))
        );
        assert_eq!(
            m.apply2(&Seed { i: 5, len: 10 }),
            Some((Seed { i: 100, len: 5 }, vec![Seed { i: 5, len: 5 }]))
        );
        assert_eq!(
            m.apply2(&Seed { i: 5, len: 20 }),
            Some((
                Seed { i: 100, len: 10 },
                vec![Seed { i: 5, len: 5 }, Seed { i: 20, len: 5 }]
            ))
        );
    }
}
