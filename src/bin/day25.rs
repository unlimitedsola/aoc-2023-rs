use petgraph::prelude::UnGraphMap;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

use aoc_2023_rust::aoc;

fn main() {
    let aoc = aoc().unwrap();
    let input = aoc.read_input(25).unwrap();
    let g = parse(&input);
    println!("part1: {}", part1(&g));
}

type G<'a> = UnGraphMap<&'a str, ()>;

fn parse(input: &str) -> G {
    UnGraphMap::from_edges(input.lines().flat_map(|line| {
        let (src, dst) = line.split_once(": ").unwrap();
        dst.split_ascii_whitespace().map(move |dst| (src, dst))
    }))
}

fn part1(g: &G) -> usize {
    let (cut, partition) = stoer_wagner_min_cut(g, |_| anyhow::Ok(1)).unwrap().unwrap();
    assert_eq!(cut, 3);
    partition.len() * (g.node_count() - partition.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test() {
        let g = parse(INPUT);
        assert_eq!(part1(&g), 54);
    }
}
