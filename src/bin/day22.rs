use std::cmp::{max, min};

use itertools::{Either, Itertools};

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(22).unwrap();
    let boxes = parse_input(&input);
    println!("part1: {}", part1(&boxes));
    println!("part2: {}", part2(&boxes));
}

type HeightMap = grid::Grid<usize>;

fn parse_input(input: &str) -> Vec<[[usize; 3]; 2]> {
    let mut boxes = input.lines().map(parse_box).collect_vec();
    boxes.sort_by_cached_key(|[[_, _, z1], [_, _, z2]]| min(*z1, *z2));
    drop(&mut boxes, None);
    boxes
}

fn parse_box(line: &str) -> [[usize; 3]; 2] {
    let coords: [&str; 2] = line.split_once('~').unwrap().into();
    coords.map(parse_coord)
}

fn parse_coord(coord: &str) -> [usize; 3] {
    coord
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect_vec()
        .try_into()
        .unwrap()
}

fn size(boxes: &[[[usize; 3]; 2]]) -> [usize; 2] {
    [
        boxes
            .iter()
            .flatten()
            .map(|x| x[0])
            .max()
            .unwrap_or_default()
            + 1,
        boxes
            .iter()
            .flatten()
            .map(|x| x[1])
            .max()
            .unwrap_or_default()
            + 1,
    ]
}

fn part1(boxes: &[[[usize; 3]; 2]]) -> usize {
    (0..boxes.len())
        .filter(|&i| drop(&mut boxes.to_vec(), Some(i)) == 0)
        .count()
}

fn part2(boxes: &[[[usize; 3]; 2]]) -> usize {
    (0..boxes.len())
        .map(|i| drop(&mut boxes.to_vec(), Some(i)))
        .sum()
}

fn drop(boxes: &mut [[[usize; 3]; 2]], i: Option<usize>) -> usize {
    let size = size(boxes);
    let mut hmap = HeightMap::init(size[1], size[0], 0);
    let mut fall_cnt = 0;
    for (ii, [[x1, y1, z1], [x2, y2, z2]]) in boxes.iter_mut().enumerate() {
        if let Some(i) = i {
            if i == ii {
                continue;
            }
        }
        let peak = bi_closed_range(*x1, *x2)
            .cartesian_product(bi_closed_range(*y1, *y2))
            .map(|pos| hmap[pos])
            .max()
            .unwrap_or_default();
        let fall_dis = min(*z1, *z2) - peak - 1;
        if fall_dis > 0 {
            fall_cnt += 1;
        }
        *z1 -= fall_dis;
        *z2 -= fall_dis;
        bi_closed_range(*x1, *x2)
            .cartesian_product(bi_closed_range(*y1, *y2))
            .for_each(|pos| hmap[pos] = max(*z1, *z2))
    }
    fall_cnt
}

fn bi_closed_range(start: usize, end: usize) -> impl Iterator<Item = usize> + Clone {
    if start < end {
        Either::Left(start..=end)
    } else {
        Either::Right((end..=start).rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test() {
        let boxes = parse_input(INPUT);
        assert_eq!(part1(&boxes), 5);
        assert_eq!(part2(&boxes), 7);
    }
}
