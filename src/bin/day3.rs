use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(3).unwrap();
    part1(&input);
}

#[derive(Debug)]
struct Num {
    num: u32,
    x: usize,
    y: usize,
    len: usize,
    found: bool,
}

fn part1(input: &str) {
    let mut nums: Vec<Num> = vec![];
    input.lines().enumerate().for_each(|(y, line)| parse_numbers(y, line, &mut nums));
    input.lines().enumerate().for_each(|(y, line)| {
        line.match_indices(|c: char| !c.is_numeric() && c != '.').for_each(|(x, _)| {
            mark_found(x, y, &mut nums);
        });
    });
    let sum: u32 = nums.iter().filter(|num| num.found).map(|num| num.num).sum();
    println!("part1: {sum}")
}

fn parse_numbers(y: usize, line: &str, nums: &mut Vec<Num>) {
    let mut chars = line.char_indices();
    while let Some((i, _)) = chars.find(|(_, c)| c.is_numeric()) {
        let e = match chars.find(|(_, c)| !c.is_numeric()) {
            Some((j, _)) => j,
            None => line.len(),
        };
        let num = line[i..e].parse().unwrap();
        nums.push(Num {
            num,
            x: i,
            y,
            len: e - i,
            found: false,
        });
    }
}

fn mark_found(x: usize, y: usize, nums: &mut [Num]) {
    nums.iter_mut().filter(|num| !num.found).for_each(|num| {
        let by = y == num.y || y + 1 == num.y || num.y + 1 == y; // same line, above, below
        let bx = x + 1 == num.x || x == num.x + num.len - 1 || (x >= num.x && x <= num.x + num.len); // before, after, inside
        if by && bx {
            num.found = true;
        }
    });
}
