use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(12).unwrap();
    part1(&input);
}

fn part1(input: &str) {
    let sum: usize = input.lines().map(solve1).sum();
    println!("part1: {}", sum)
}

fn solve1(line: &str) -> usize {
    let (input, counts) = line.split_once(' ').unwrap();
    let counts = counts.split(',').map(|c| c.parse::<usize>().unwrap()).collect_vec();
    let pattern = gen_pattern(&counts);
    let mut state = vec![0usize; pattern.len()];
    state[0] = 1;
    for c in input.chars() {
        state = next_state(&state, &pattern, c);
    }
    state.iter().rev().take(2).sum()
}

fn gen_pattern(counts: &[usize]) -> String {
    let mut state = ".".to_string();
    for count in counts {
        for _ in 0..*count {
            state.push('#');
        }
        state.push('.');
    }
    state
}

fn next_state(state: &[usize], pattern: &str, input: char) -> Vec<usize> {
    let mut next_state = vec![0usize; state.len()];
    for i in 0..state.len() {
        let mut chars = pattern.chars().skip(i);
        let cur = chars.next().unwrap();
        let next = chars.next();
        match (cur, next, input) {
            (_, None, '#') => {}
            (_, None, _) => next_state[i] += state[i],
            ('.', Some('.'), '.') => next_state[i] += state[i],
            ('.', Some('.'), '#') => {}
            ('.', Some('.'), '?') => next_state[i] += state[i],
            ('.', Some('#'), '.') => next_state[i] += state[i],
            ('.', Some('#'), '#') => next_state[i + 1] += state[i],
            ('.', Some('#'), '?') => {
                next_state[i] += state[i];
                next_state[i + 1] += state[i];
            }
            ('#', Some('.'), '.') => next_state[i + 1] += state[i],
            ('#', Some('.'), '#') => {}
            ('#', Some('.'), '?') => next_state[i + 1] += state[i],
            ('#', Some('#'), '.') => {}
            ('#', Some('#'), '#') => next_state[i + 1] += state[i],
            ('#', Some('#'), '?') => next_state[i + 1] += state[i],
            _ => unreachable!(),
        }
    }
    next_state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve1(".??..?##? 1,3"), 4);
        assert_eq!(solve1("???.### 1,1,3"), 1);
        assert_eq!(solve1(".??..??...?##. 1,1,3"), 4);
        assert_eq!(solve1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(solve1("????.#...#... 4,1,1"), 1);
        assert_eq!(solve1("????.######..#####. 1,6,5"), 4);
        assert_eq!(solve1("?###???????? 3,2,1"), 10);
        assert_eq!(solve1("#..?#?###???? 1,8,1"), 1);
        assert_eq!(solve1(".?#?..?#?...?####?... 1,1,4"), 1);
        assert_eq!(solve1("???#.???# 1,1"), 1);
    }
}
