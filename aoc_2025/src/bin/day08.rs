use std::collections::{BTreeSet, HashSet, VecDeque};

use aoc_utils::{input, parser::*};
use itertools::Itertools;

fn square_dst(p1: &[usize], p2: &[usize]) -> usize {
    p1.iter().zip(p2).map(|(a, b)| (b - a).pow(2)).sum()
}

fn groups_and_edges(
    positions: &[Vec<usize>],
) -> (
    VecDeque<HashSet<&Vec<usize>>>,
    BTreeSet<(usize, &Vec<usize>, &Vec<usize>)>,
) {
    let mut groups = VecDeque::new();
    let mut edges = BTreeSet::new();
    for i in 0..positions.len() {
        groups.push_back({
            let mut s = HashSet::new();
            s.insert(&positions[i]);
            s
        });
        for j in i + 1..positions.len() {
            edges.insert((
                square_dst(&positions[i], &positions[j]),
                &positions[i],
                &positions[j],
            ));
        }
    }
    (groups, edges)
}

fn merge_closest<'a>(
    groups: &mut VecDeque<HashSet<&'a Vec<usize>>>,
    edges: &mut BTreeSet<(usize, &'a Vec<usize>, &'a Vec<usize>)>,
) -> (&'a Vec<usize>, &'a Vec<usize>) {
    let (_, a, b) = edges.pop_first().unwrap();
    let mut acc = HashSet::new();
    for _ in 0..groups.len() {
        let s = groups.pop_front().unwrap();
        if s.contains(&a) || s.contains(&b) {
            for k in s {
                acc.insert(k);
            }
        } else {
            groups.push_back(s);
        }
    }
    groups.push_back(acc);

    (&a, &b)
}

fn main() {
    let positions = input::parse_lines(number().sep_by(prefix(","), 3..=3));

    let (mut groups, mut edges) = groups_and_edges(&positions);

    for _ in 0..1000 {
        merge_closest(&mut groups, &mut edges);
    }

    let part1 = groups
        .iter()
        .map(|g| g.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>();
    println!("{part1}");

    let (mut groups, mut edges) = groups_and_edges(&positions);

    let part2 = loop {
        let (a, b) = merge_closest(&mut groups, &mut edges);
        if groups.len() == 1 {
            break a[0] * b[0];
        }
    };
    println!("{part2}");
}
