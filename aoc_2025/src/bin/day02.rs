use aoc_utils::{input, parser::*};

fn is_invalid(n: usize) -> bool {
    let s = n.to_string();
    s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..]
}

fn is_invalid2(n: usize) -> bool {
    let chars = n.to_string().chars().collect::<Vec<_>>();
    (1..=chars.len() / 2).any(|i| chars.chunks(i).all(|c| c == &chars[..i]))
}

fn main() {
    let nums = input::parse_input(
        number()
            .then_zip_with(prefix("-") >> number(), |a, b| a..=b)
            .sep_by(prefix(","), 1..),
    )
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let part1 = nums.iter().filter(|n| is_invalid(**n)).sum::<usize>();
    println!("{part1}");

    let part2 = nums.iter().filter(|n| is_invalid2(**n)).sum::<usize>();
    println!("{part2}");
}
