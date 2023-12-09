use itertools::Itertools;
use num::Zero;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(9).unwrap();
    let nums = input.lines()
        .map(|l| l.split_ascii_whitespace())
        .map(|nums| nums.map(|n| n.parse::<i64>().unwrap()).collect_vec()).collect_vec();
    part1(&nums);
    part2(&nums);
}

fn part1(nums: &[Vec<i64>]) {
    let sum = nums.iter().cloned().map(extrapolate1).sum::<i64>();
    println!("part 1: {}", sum);
}

fn extrapolate1(mut nums: Vec<i64>) -> i64 {
    let mut lasts = vec![];
    while nums.iter().any(|n| !n.is_zero()) {
        for i in 0..nums.len() - 1 {
            nums[i] = nums[i + 1] - nums[i];
        }
        lasts.push(nums.pop().unwrap());
    }
    lasts.iter().sum()
}

fn part2(nums: &[Vec<i64>]) {
    let sum = nums.iter().cloned().map(extrapolate2).sum::<i64>();
    println!("part 2: {}", sum);
}

fn extrapolate2(mut nums: Vec<i64>) -> i64 {
    let mut firsts = vec![];
    while nums.iter().any(|n| !n.is_zero()) {
        firsts.push(nums[0]);
        for i in 0..nums.len() - 1 {
            nums[i] = nums[i + 1] - nums[i];
        }
        nums.pop().unwrap();
    }
    firsts.reverse();
    firsts.iter().fold(0, |acc, n| n - acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        assert_eq!(extrapolate2(vec![10, 13, 16, 21, 30, 45]), 5);
    }
}
