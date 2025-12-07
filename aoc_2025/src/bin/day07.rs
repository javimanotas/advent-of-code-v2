use aoc_utils::{input, parser::*};

use std::collections::{HashMap, HashSet};

fn count_splits(mtx: &[Vec<char>], pos: (usize, usize), unique: &mut HashSet<(usize, usize)>) {
    if unique.contains(&pos) || pos.0 >= mtx.len() {
        return;
    }

    if mtx[pos.0][pos.1] == '^' {
        unique.insert(pos);
        count_splits(mtx, (pos.0, pos.1 - 1), unique);
        count_splits(mtx, (pos.0, pos.1 + 1), unique);
    } else {
        count_splits(mtx, (pos.0 + 1, pos.1), unique);
    }
}

fn count_splits_2(
    mtx: &[Vec<char>],
    pos: (usize, usize),
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if pos.0 >= mtx.len() {
        1
    } else if mtx[pos.0][pos.1] == '^' {
        if let Some(v) = cache.get(&pos) {
            *v
        } else {
            let cnt = count_splits_2(mtx, (pos.0, pos.1 - 1), cache)
                + count_splits_2(mtx, (pos.0, pos.1 + 1), cache);
            cache.insert(pos, cnt);
            cnt
        }
    } else {
        count_splits_2(mtx, (pos.0 + 1, pos.1), cache)
    }
}

fn main() {
    let mtx = input::parse_lines(any_char().repeat(..));
    let start = (
        0,
        mtx[0]
            .iter()
            .enumerate()
            .find_map(|(i, c)| (*c == 'S').then_some(i))
            .unwrap(),
    );

    let mut unique = HashSet::new();
    count_splits(&mtx, start, &mut unique);
    let part1 = unique.len();
    println!("{part1}");

    let part2 = count_splits_2(&mtx, start, &mut HashMap::new());
    println!("{part2}");
}
