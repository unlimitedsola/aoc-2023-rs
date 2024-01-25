use itertools::Itertools;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(7).unwrap();
    solve(&input, false);
    solve(&input, true);
}

fn solve(input: &str, p2: bool) {
    let ans: u64 = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(h, b)| {
            (
                Hand::parse(h.chars().collect_vec().try_into().unwrap(), p2),
                b.parse::<u64>().unwrap(),
            )
        })
        .sorted_by_cached_key(|(Hand(t, c), _)| (*t, pos(c, p2)))
        .enumerate()
        .map(|(i, (_, b))| b * (i + 1) as u64)
        .sum();
    println!("part{}: {}", if p2 { 2 } else { 1 }, ans)
}

const P1: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const P2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn pos(c: &[char; 5], p2: bool) -> [usize; 5] {
    c.map(|c| {
        if p2 { P2 } else { P1 }
            .iter()
            .position(|p| *p == c)
            .unwrap()
    })
}

#[derive(Debug)]
struct Hand(HandType, [char; 5]);

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn parse(s: [char; 5], p2: bool) -> Self {
        let mut counts = s.iter().counts();
        let js = p2.then(|| counts.remove(&'J')).flatten();
        let mut counts = counts.into_values().collect_vec();

        counts.sort();
        counts.reverse();

        if let Some(j) = js {
            if let Some(c) = counts.first_mut() {
                *c += j;
            } else {
                counts.insert(0, 5)
            }
        }

        let t = match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("invalid hand: {:?}", counts),
        };
        Hand(t, s)
    }
}
