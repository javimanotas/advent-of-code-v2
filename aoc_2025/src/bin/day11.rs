use aoc_utils::{input, parser::*};

use std::collections::HashMap;

fn different_paths<'a>(node: &'a str, graph: &HashMap<&'a str, Vec<&'a str>>) -> usize {
    if node == "out" {
        1
    } else {
        graph
            .get(node)
            .unwrap()
            .iter()
            .map(|n| different_paths(n, graph))
            .sum()
    }
}

fn main() {
    let ids = input::parse_lines(
        satisfies(char::is_lowercase)
            .repeat(1..)
            .collect_string()
            .anywhere()
            .repeat(2..),
    );

    let mut map: HashMap<&str, _> = HashMap::new();
    for i in &ids {
        let mut v: Vec<&str> = vec![];
        for j in &i[1..] {
            v.push(j);
        }
        map.insert(&i[0], v);
    }

    let part1 = different_paths("you", &map);
    println!("{part1}");
}
