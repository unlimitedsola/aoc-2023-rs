use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(15).unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input.trim().split(',').map(hash).sum()
}

fn hash(str: &str) -> usize {
    let mut h = 0;
    for c in str.chars() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }
    h
}

enum Instruction<'a> {
    Dash(&'a str),
    Equal(&'a str, usize),
}

impl<'a> Instruction<'a> {
    fn parse(str: &'a str) -> Self {
        if str.ends_with('-') {
            Self::Dash(str.trim_end_matches('-'))
        } else {
            let (l, r) = str.split_once('=').unwrap();
            Self::Equal(l, r.parse::<usize>().unwrap())
        }
    }
}

fn part2(input: &str) -> usize {
    let mut boxes = vec![vec![]; 256];
    let instructions = input
        .trim()
        .split(',')
        .map(Instruction::parse)
        .collect_vec();
    for inst in instructions {
        match inst {
            Instruction::Dash(l) => {
                let h = hash(l);
                boxes[h].retain(|(ll, _)| *ll != l);
            }
            Instruction::Equal(l, f) => {
                let h = hash(l);
                let mut found = false;
                boxes[h].iter_mut().for_each(|(ll, ff)| {
                    if *ll == l {
                        *ff = f;
                        found = true;
                    }
                });
                if !found {
                    boxes[h].push((l, f));
                }
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .flat_map(|(b, v)| v.iter().enumerate().map(move |(s, (_, f))| (b, s, f)))
        .map(|(b, s, f)| (b + 1) * (s + 1) * f)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), 1320)
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), 145)
    }
}
