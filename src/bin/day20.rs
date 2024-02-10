use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::{Deref, DerefMut};

use itertools::Itertools;
use num::integer::lcm;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(20).unwrap();
    let modules = parse_modules(&input);
    println!("part1: {}", part1(&modules));
    let modules = parse_modules(&input);
    println!("part2: {}", part2(&modules));
}

fn parse_modules(input: &str) -> HashMap<&str, Module> {
    let modules: HashMap<_, _> = input
        .lines()
        .map(Module::parse)
        .map(|m| (m.name, m))
        .collect();
    for (src, m) in modules.iter() {
        for dst in &m.dst {
            if let Some(m) = modules.get(dst) {
                if let ModuleType::Conjunction { states } = m.typ.borrow_mut().deref_mut() {
                    states.insert(src, false);
                }
            }
        }
    }
    modules
}

fn part1(modules: &HashMap<&str, Module>) -> usize {
    let mut cnt = Counter::default();
    for _ in 0..1000 {
        pulse1(modules, &mut cnt);
    }
    cnt.low * cnt.high
}

fn pulse1(mm: &HashMap<&str, Module>, cnt: &mut Counter) {
    let mut q = VecDeque::new();
    q.push_back(("button", "broadcaster", false));
    while let Some((src, dst, high)) = q.pop_front() {
        if high {
            cnt.high += 1;
        } else {
            cnt.low += 1;
        }
        if let Some(m) = mm.get(dst) {
            let output = m.typ.borrow_mut().output(src, high);
            if let Some(output) = output {
                let src = dst;
                for dst in &m.dst {
                    q.push_back((src, dst, output));
                }
            }
        }
    }
}

fn part2(modules: &HashMap<&str, Module>) -> usize {
    impl<'a> ModuleType<'a> {
        fn conj_states(&self) -> &HashMap<&'a str, bool> {
            match self {
                ModuleType::Conjunction { states } => states,
                _ => unreachable!(),
            }
        }
    }
    let rx_conj = &modules
        .values()
        .filter(|m| m.dst.contains(&"rx"))
        .exactly_one()
        .unwrap();
    let mut rx_cnt = rx_conj
        .typ
        .borrow()
        .conj_states()
        .keys()
        .map(|&k| (k, 0usize))
        .collect::<HashMap<_, _>>();
    let mut cnt = 0usize;
    loop {
        cnt += 1;
        pulse2(modules, cnt, &mut rx_cnt);
        if rx_cnt.values().all(|v| *v != 0) {
            return rx_cnt.values().fold(1, |acc, v| lcm(acc, *v));
        }
    }
}

fn pulse2<'a>(mm: &'a HashMap<&'a str, Module>, cnt: usize, rx_cnt: &mut HashMap<&'a str, usize>) {
    let mut q = VecDeque::new();
    q.push_back(("button", "broadcaster", false));
    while let Some((src, dst, high)) = q.pop_front() {
        if high {
            if let Some(c) = rx_cnt.get_mut(src) {
                if *c == 0 {
                    *c = cnt;
                }
            }
        }
        if let Some(m) = mm.get(dst) {
            let output = m.typ.borrow_mut().output(src, high);
            if let Some(output) = output {
                let src = dst;
                for dst in &m.dst {
                    q.push_back((src, dst, output));
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct Counter {
    low: usize,
    high: usize,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    typ: RefCell<ModuleType<'a>>,
    dst: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn parse(str: &'a str) -> Self {
        let (m, dst) = str.split_once(" -> ").unwrap();
        let dst = dst.split(", ").collect_vec();
        let typ = m
            .chars()
            .next()
            .map(ModuleType::parse)
            .map(RefCell::new)
            .unwrap();
        let name = m.trim_start_matches(|c| c == '%' || c == '&');
        Self { name, typ, dst }
    }
}

#[derive(Debug)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { states: HashMap<&'a str, bool> },
}

impl<'a> ModuleType<'a> {
    fn parse(c: char) -> Self {
        match c {
            '%' => Self::FlipFlop { state: false },
            '&' => Self::Conjunction {
                states: HashMap::new(),
            },
            _ => Self::Broadcaster,
        }
    }

    fn output(&mut self, src: &'a str, high: bool) -> Option<bool> {
        match self {
            ModuleType::Broadcaster => Some(high),
            ModuleType::FlipFlop { state } => {
                if high {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            ModuleType::Conjunction { states } => {
                states.insert(src, high);
                Some(!states.values().all(|&v| v))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test() {
        let input1 = parse_modules(INPUT1);
        let input2 = parse_modules(INPUT2);
        assert_eq!(part1(&input1), 32000000);
        assert_eq!(part1(&input2), 11687500);
    }
}
