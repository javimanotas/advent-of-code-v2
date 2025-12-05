use aoc_utils::{input, parser::*};

fn get_fresh(ranges: &mut [(usize, usize)]) -> usize {
    ranges.sort();
    let mut intersections = vec![ranges[0]];

    for r in ranges[1..].iter() {
        let last = intersections.len() - 1;
        if intersections[last].1 >= r.0 {
            intersections[last] = (intersections[last].0, intersections[last].1.max(r.1))
        } else {
            intersections.push(*r);
        }
    }

    intersections.iter().map(|(a, b)| b - a + 1).sum()
}

fn main() {
    let mut ranges = input::parse_ok_lines(number() + (prefix("-") >> number()));
    let nums = input::parse_ok_lines(number() << eof());

    let part1 = nums
        .iter()
        .filter(|n| ranges.iter().any(|(a, b)| *n >= a && *n <= b))
        .count();
    println!("{part1}");

    let part2 = get_fresh(&mut ranges);
    println!("{part2}");
}
