use std::ops::Neg;

use itertools::Itertools;
use nalgebra::{vector, SMatrix, Vector2, Vector3, Vector6};
use num::Zero;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(24).unwrap();
    let stones = parse(&input);
    println!(
        "part1: {}",
        part1(&stones, &vector![200000000000000f64, 400000000000000f64])
    );
    println!("part2: {}", part2(&stones));
}

struct Hailstone {
    p: Vector3<f64>,
    v: Vector3<f64>,
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").unwrap();

            fn parse_v3(s: &str) -> Vector3<f64> {
                Vector3::from_iterator(s.split(", ").map(|v| v.parse().unwrap()))
            }
            Hailstone {
                p: parse_v3(p),
                v: parse_v3(v),
            }
        })
        .collect_vec()
}

// y = ax + b
// a = vy / vx
// b = y - ax

// a1x + b1 = a2x + b2
// a1x - a2x = b2 - b1
// x(a1 - a2) = b2 - b1
// x = (b2 - b1) / (a1 - a2)
fn collide_2d(a: &Hailstone, b: &Hailstone, bound: &Vector2<f64>) -> Option<Vector2<f64>> {
    let a1 = a.v.y / a.v.x;
    let b1 = a.p.y - a1 * a.p.x;
    let a2 = b.v.y / b.v.x;
    let b2 = b.p.y - a2 * b.p.x;

    if a1 == a2 {
        return None;
    }

    let x = (b2 - b1) / (a1 - a2);
    let y = a1 * x + b1;
    let t1 = (x - a.p.x) / a.v.x;
    let t2 = (x - b.p.x) / b.v.x;

    if t1 < 0.0 || t2 < 0.0 || x < bound.x || x > bound.y || y < bound.x || y > bound.y {
        return None;
    }
    Some(vector![x, y])
}

fn part1(stones: &[Hailstone], bound: &Vector2<f64>) -> usize {
    stones
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| collide_2d(a, b, bound).is_some())
        .count()
}

fn part2(stones: &[Hailstone]) -> f64 {
    // P + t[i] * V = p[i] + t[i] * v[i]
    // P - p[i] = t[i] * (v[i] - V)
    // cross product both sides with (v[i] - V)
    // (P - p[i]) x (v[i] - V) = t[i] * (v[i] - V) x (v[i] - V)
    // because t[i] is a scalar value and the cross product of two parallel vectors is 0
    // the right side of the equation is 0
    // (P - p[i]) x (v[i] - V) = 0
    // P x (v[i] - V) - p[i] x (v[i] - V) = 0
    // P x v[i] - P x V - p[i] x v[i] + p[i] x V = 0
    // For every i, j:
    // P x v[i] - P x V - p[i] x v[i] + p[i] x V = P x v[j] - P x V - p[j] x v[j] + p[j] x V
    // simplify and moving unknowns to one side
    // -(v[i] - v[j]) x P + (p[i] - p[j]) x V = p[i] x v[i] - p[j] x v[j]
    // making a matrix of the unknowns out of two known pairs (i, j) and (i, k)
    // M * x = b
    // | -(v[i] - v[j]), p[i] - p[j] | | P | = | p[i] x v[i] - p[j] x v[j] |
    // | -(v[i] - v[k]), p[i] - p[k] | | V | = | p[i] x v[i] - p[k] x v[k] |
    // x = M^-1 * b

    let mut m = SMatrix::<f64, 6, 6>::zero();
    m.fixed_view_mut::<3, 3>(0, 0)
        .copy_from(&(stones[0].v - stones[1].v).neg().cross_matrix());
    m.fixed_view_mut::<3, 3>(0, 3)
        .copy_from(&(stones[0].p - stones[1].p).cross_matrix());
    m.fixed_view_mut::<3, 3>(3, 0)
        .copy_from(&(stones[0].v - stones[2].v).neg().cross_matrix());
    m.fixed_view_mut::<3, 3>(3, 3)
        .copy_from(&(stones[0].p - stones[2].p).cross_matrix());
    let mut b = Vector6::zero();
    b.fixed_view_mut::<3, 1>(0, 0)
        .copy_from(&(stones[0].p.cross(&stones[0].v) - stones[1].p.cross(&stones[1].v)));
    b.fixed_view_mut::<3, 1>(3, 0)
        .copy_from(&(stones[0].p.cross(&stones[0].v) - stones[2].p.cross(&stones[2].v)));

    m.lu().solve_mut(&mut b);
    let p = b.fixed_view::<3, 1>(0, 0);
    let v = b.fixed_view::<3, 1>(3, 0);

    dbg!(p.iter().collect_vec());
    dbg!(v.iter().collect_vec());

    p.iter().map(|x| x.round()).sum::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";

    #[test]
    fn test() {
        let stones = parse(INPUT);
        assert_eq!(part1(&stones, &vector![7.0, 27.0]), 2);
        assert_eq!(part2(&stones), 47.0);
    }
}
