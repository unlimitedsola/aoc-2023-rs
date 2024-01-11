use std::iter;

use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(12).unwrap();
    part1(&input);
    part2(&input);
}

fn parse(input: &str) -> impl Iterator<Item=(&str, Vec<usize>)> {
    input.lines()
        .map(|line| {
            let (input, counts) = line.split_once(' ').unwrap();
            let counts = counts.split(',').map(|c| c.parse::<usize>().unwrap()).collect_vec();
            (input, counts)
        })
}

fn part1(input: &str) {
    let sum: usize = parse(input)
        .map(|(input, counts)| solve(input, &counts))
        .sum();
    println!("part1: {}", sum)
}

fn part2(input: &str) {
    let sum: usize = parse(input)
        .map(|(input, counts)| expand(input, &counts))
        .map(|(input, counts)| solve(&input, &counts))
        .sum();
    println!("part2: {}", sum)
}

fn expand(input: &str, counts: &[usize]) -> (String, Vec<usize>) {
    (iter::repeat(input).take(5).join("?"), counts.repeat(5))
}

fn solve(input: &str, counts: &[usize]) -> usize {
    let pattern = gen_pattern(counts);
    let mut state = vec![0usize; pattern.len()];
    state[0] = 1;
    for c in input.chars() {
        state = next_state(&state, &pattern, c);
    }
    state.iter().rev().take(2).sum()
}

fn gen_pattern(counts: &[usize]) -> Vec<char> {
    let mut state = vec!['.'];
    for count in counts {
        for _ in 0..*count {
            state.push('#');
        }
        state.push('.');
    }
    state
}

fn next_state(state: &[usize], pattern: &[char], input: char) -> Vec<usize> {
    let mut new_state = vec![0usize; state.len()];
    for i in 0..state.len() {
        let cur = pattern[i];
        let next = pattern.get(i + 1);
        match (cur, next, input) {
            (_, None, '#') => {}
            (_, None, _) => new_state[i] += state[i],
            ('.', Some('.'), '.') => new_state[i] += state[i],
            ('.', Some('.'), '#') => {}
            ('.', Some('.'), '?') => new_state[i] += state[i],
            ('.', Some('#'), '.') => new_state[i] += state[i],
            ('.', Some('#'), '#') => new_state[i + 1] += state[i],
            ('.', Some('#'), '?') => {
                new_state[i] += state[i];
                new_state[i + 1] += state[i];
            }
            ('#', Some('.'), '.') => new_state[i + 1] += state[i],
            ('#', Some('.'), '#') => {}
            ('#', Some('.'), '?') => new_state[i + 1] += state[i],
            ('#', Some('#'), '.') => {}
            ('#', Some('#'), '#') => new_state[i + 1] += state[i],
            ('#', Some('#'), '?') => new_state[i + 1] += state[i],
            _ => unreachable!(),
        }
    }
    new_state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve(".??..?##?", &[1, 3]), 4);
        assert_eq!(solve("???.###", &[1, 1, 3]), 1);
        assert_eq!(solve(".??..??...?##.", &[1, 1, 3]), 4);
        assert_eq!(solve("?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
        assert_eq!(solve("????.#...#...", &[4, 1, 1]), 1);
        assert_eq!(solve("????.######..#####.", &[1, 6, 5]), 4);
        assert_eq!(solve("?###????????", &[3, 2, 1]), 10);
        assert_eq!(solve("#..?#?###????", &[1, 8, 1]), 1);
        assert_eq!(solve(".?#?..?#?...?####?...", &[1, 1, 4]), 1);
        assert_eq!(solve("???#.???#", &[1, 1]), 1);
    }

    #[test]
    fn part2() {
        assert_eq!(solve2("???.###", &[1, 1, 3]), 1);
        assert_eq!(solve2("?###????????", &[3, 2, 1]), 506250);
    }

    fn solve2(input: &str, counts: &[usize]) -> usize {
        let (input, counts) = expand(input, counts);
        solve(&input, &counts)
    }
}
