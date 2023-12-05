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
        // I gave up optimizing this!
        if (seed.i..seed.i + seed.len).contains(&self.src) { // seed starts before mapping
            if seed.i + seed.len > self.src + self.len { // seed ends after mapping
                let intersect = Seed {
                    i: self.dst,
                    len: self.len,
                };
                let trail = Seed {
                    i: seed.i + self.len, // starts after intersect
                    len: seed.i + seed.len - self.src - self.len, // seed.end - mapping.end
                };
                if seed.i == self.src { // no head
                    Some((intersect, vec![trail]))
                } else {
                    let head = Seed {
                        i: seed.i,
                        len: self.src - seed.i,
                    };
                    Some((intersect, vec![head, trail]))
                }
            } else {
                let intersect = Seed {
                    i: self.dst,
                    len: seed.i + seed.len - self.src,
                };
                if seed.i == self.src { // no head
                    Some((intersect, vec![]))
                } else {
                    let head = Seed {
                        i: seed.i,
                        len: self.src - seed.i,
                    };
                    Some((intersect, vec![head]))
                }
            }
        } else if (self.src..self.src + self.len).contains(&seed.i) { // seed starts within mapping
            if seed.i + seed.len > self.src + self.len { // seed ends after mapping
                let intersect = Seed {
                    i: self.dst + seed.i - self.src,
                    len: self.len + self.src - seed.i,
                };
                let trail = Seed {
                    i: self.src + self.len,
                    len: seed.i + seed.len - self.src - self.len,
                };
                Some((intersect, vec![trail]))
            } else {
                let intersect = Seed {
                    i: self.dst + seed.i - self.src,
                    len: seed.len,
                };
                Some((intersect, vec![]))
            }
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

#[derive(Debug, Copy, Clone)]
struct Seed {
    i: u64,
    len: u64,
}

fn parse_seeds2(line: &str) -> Vec<Seed> {
    line.strip_prefix("seeds: ").unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .tuples()
        .map(|(i, len)| Seed { i, len })
        .collect()
}

fn part2<I>(mut seeds: Vec<Seed>, maps: &[I]) where I: AsRef<[Mapping]> {
    for map in maps {
        seeds = do_map(seeds, map.as_ref());
    }
    seeds.sort_by_key(|s| s.i);
    println!("part2: {}", seeds.first().unwrap().i)
}

fn do_map(mut seeds: Vec<Seed>, map: &[Mapping]) -> Vec<Seed> {
    let mut res = vec![];
    while let Some(seed) = seeds.pop() {
        for mapping in map {
            if let Some((mapped, rem)) = mapping.apply2(&seed) {
                res.push(mapped);
                seeds.extend(rem)
            }
        }
    }
    res
}
