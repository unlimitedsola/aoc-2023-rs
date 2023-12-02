use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(2).unwrap();
    part1(&input);
}

fn part1(input: &str) {
    let mut sum = 0u32;
    'game: for line in input.lines() {
        let (header, sets) = line.split_once(": ").unwrap();
        let gid: u32 = header["Game ".len()..].parse().unwrap();
        for set in sets.split("; ") {
            for cubes in set.split(", ") {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count: u32 = count.parse().unwrap();
                let valid = match color {
                    "red" if count <= 12 => true,
                    "green" if count <= 13 => true,
                    "blue" if count <= 14 => true,
                    _ => false
                };
                if !valid {
                    continue 'game
                }
            }
        }
        sum += gid;
    }
    println!("part1: {sum}")
}
