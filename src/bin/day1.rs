use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(1).unwrap();
    part1(&input)
}

fn part1(input: &str) {
    let sum: u32 = input.lines()
        .map(|l| {
            let mut digits = l.chars().filter(char::is_ascii_digit);
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            (first, last)
        })
        .map(|(a, b)| a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap())
        .sum();
    println!("part1: {sum}")
}
