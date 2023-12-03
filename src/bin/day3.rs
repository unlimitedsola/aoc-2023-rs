use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(3).unwrap();

    let mut nums = Num::parse(&input);
    part1(&input, &mut nums);
    part2(&input, &nums);
}

#[derive(Debug)]
struct Num {
    num: u32,
    x: usize,
    y: usize,
    len: usize,
    found: bool,
}

impl Num {
    fn parse(input: &str) -> Vec<Num> {
        let mut nums: Vec<Num> = vec![];
        input.lines().enumerate().for_each(|(y, line)| Self::parse_line(y, line, &mut nums));
        nums
    }

    fn parse_line(y: usize, line: &str, nums: &mut Vec<Num>) {
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

    fn touches(&self, x: usize, y: usize) -> bool {
        let by = y == self.y || y + 1 == self.y || self.y + 1 == y; // same line, above, below
        let bx = x + 1 == self.x || x == self.x + self.len - 1 || (x >= self.x && x <= self.x + self.len); // before, after, inside
        by && bx
    }
}

fn part1(input: &str, nums: &mut [Num]) {
    input.lines().enumerate().for_each(|(y, line)| {
        line.match_indices(|c: char| !c.is_numeric() && c != '.').for_each(|(x, _)| {
            nums.iter_mut()
                .filter(|num| !num.found)
                .filter(|num| num.touches(x, y))
                .for_each(|num| num.found = true);
        });
    });
    let sum: u32 = nums.iter().filter(|num| num.found).map(|num| num.num).sum();
    println!("part1: {sum}")
}

fn part2(input: &str, nums: &[Num]) {
    let mut sum = 0u32;
    input.lines().enumerate().for_each(|(y, line)| {
        line.match_indices('*').for_each(|(x, _)| {
            let mut parts = nums.iter().filter(|num| num.touches(x, y));
            let a = parts.next();
            let b = parts.next();
            if let (Some(a), Some(b)) = (a, b) {
                sum += a.num * b.num;
            }
        });
    });
    println!("part2: {sum}")
}
