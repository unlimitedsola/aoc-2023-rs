use std::collections::HashMap;
use std::ops::Index;
use std::sync::Arc;

use itertools::Itertools;
use rayon::prelude::*;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(19).unwrap();
    let (workflows, parts) = parse(&input);
    let system = System::new(workflows);
    println!("part1: {}", part1(&system, &parts));
    println!("part2: {}", part2(&system));
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let mut lines = input.lines();
    let workflows = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(Workflow::parse)
        .collect_vec();
    let parts = lines.map(Part::parse).collect_vec();
    (workflows, parts)
}

fn part1(sys: &System, parts: &[Part]) -> usize {
    parts
        .iter()
        .filter(|p| sys.accepts(p))
        .map(|p| p.sum())
        .sum()
}

fn part2(sys: &System) -> usize {
    let [x, m, a, s] = sys
        .splits()
        .map(|v| v.into_iter().tuple_windows::<(_, _)>().collect_vec())
        .map(Arc::new);
    x.par_iter()
        .map(move |&x| {
            let a = Arc::clone(&a);
            let s = Arc::clone(&s);
            m.iter()
                .map(move |&m| {
                    let s = Arc::clone(&s);
                    a.iter()
                        .map(move |&a| {
                            let s = Arc::clone(&s);
                            s.iter()
                                .map(move |&s| sys.accepts_range([x, m, a, s]))
                                .sum::<usize>()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

struct System(HashMap<String, Workflow>);

impl System {
    fn new(workflows: Vec<Workflow>) -> Self {
        Self(workflows.into_iter().map(|w| (w.name.clone(), w)).collect())
    }

    fn accepts(&self, part: &Part) -> bool {
        let mut workflow = self.0.get("in").unwrap();
        loop {
            for rule in &workflow.rules {
                if let Some(cond) = &rule.condition {
                    if !cond.eval(part) {
                        continue;
                    }
                }
                match &rule.destination {
                    Destination::Workflow(name) => {
                        workflow = self.0.get(name).unwrap();
                        break;
                    }
                    Destination::Accept => return true,
                    Destination::Reject => return false,
                }
            }
        }
    }

    fn accepts_range(&self, range: [(usize, usize); 4]) -> usize {
        let part = Part(range.map(|(l, _)| l));
        if self.accepts(&part) {
            range.iter().map(|(l, r)| r - l).product()
        } else {
            0
        }
    }

    fn conditions(&self) -> impl Iterator<Item = &Condition> {
        self.0
            .values()
            .flat_map(|w| w.rules.iter().filter_map(|r| r.condition.as_ref()))
    }

    fn splits(&self) -> [Vec<usize>; 4] {
        let mut splits = [vec![1, 4001], vec![1, 4001], vec![1, 4001], vec![1, 4001]];
        for cond in self.conditions() {
            let v = match cond.operator {
                Operator::GreaterThan => cond.value + 1,
                Operator::LessThan => cond.value,
            };
            splits[cond.source as usize].push(v);
        }
        splits.iter_mut().for_each(|v| v.sort_unstable());
        splits
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(str: &str) -> Self {
        let (name, rest) = str.split_once('{').unwrap();
        let rules = rest.strip_suffix('}').unwrap();
        let rules = rules.split(',').map(Rule::parse).collect_vec();
        Self {
            name: name.to_owned(),
            rules,
        }
    }
}

struct Rule {
    condition: Option<Condition>,
    destination: Destination,
}

impl Rule {
    fn parse(str: &str) -> Self {
        if let Some((cond, dest)) = str.split_once(':') {
            Self {
                condition: Some(Condition::parse(cond)),
                destination: Destination::parse(dest),
            }
        } else {
            Self {
                condition: None,
                destination: Destination::parse(str),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Condition {
    source: Source,
    operator: Operator,
    value: usize,
}

impl Condition {
    fn parse(str: &str) -> Self {
        let mut chars = str.chars();
        let (src, op, val) = (chars.next().unwrap(), chars.next().unwrap(), chars.as_str());
        Self {
            source: Source::parse(src),
            operator: Operator::parse(op),
            value: val.parse().unwrap(),
        }
    }

    fn eval(&self, p: &Part) -> bool {
        self.operator.eval(p[self.source], self.value)
    }
}

#[derive(Debug, Copy, Clone)]
enum Source {
    X = 0,
    M,
    A,
    S,
}

impl Source {
    fn parse(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!("invalid source: {}", c),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    GreaterThan,
    LessThan,
}

impl Operator {
    fn parse(c: char) -> Self {
        match c {
            '>' => Self::GreaterThan,
            '<' => Self::LessThan,
            _ => unreachable!("invalid operator: {}", c),
        }
    }

    fn eval(&self, a: usize, b: usize) -> bool {
        match self {
            Self::GreaterThan => a > b,
            Self::LessThan => a < b,
        }
    }
}

enum Destination {
    Workflow(String),
    Reject,
    Accept,
}

impl Destination {
    fn parse(str: &str) -> Self {
        match str {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(str.to_owned()),
        }
    }
}

struct Part([usize; 4]);

impl Part {
    fn parse(str: &str) -> Self {
        let str = str.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let values = str
            .split(',')
            .map(|s| {
                let (_, val) = s.split_once('=').unwrap();
                val.parse().unwrap()
            })
            .collect_vec()
            .try_into()
            .unwrap();
        Self(values)
    }

    fn sum(&self) -> usize {
        self.0.iter().sum()
    }
}

impl Index<Source> for Part {
    type Output = usize;

    fn index(&self, index: Source) -> &Self::Output {
        &self.0[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test() {
        let (workflows, parts) = parse(INPUT);
        let system = System::new(workflows);
        assert_eq!(part1(&system, &parts), 19114);
        assert_eq!(part2(&system), 167409079868000);
    }
}
