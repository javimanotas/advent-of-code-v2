use aoc_utils::{input, parser::*};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

fn joltage<'a>(
    digits: &'a [usize],
    n: usize,
    mut cache: &mut HashMap<(&'a [usize], usize), usize>,
) -> usize {
    if n == 0 {
        return 0;
    }

    if let Some(&n) = cache.get(&(digits, n)) {
        return n;
    }

    let max = (0..=digits.len() - n)
        .map(|i| {
            digits[i] * 10_usize.pow(n as u32 - 1) + joltage(&digits[i + 1..], n - 1, &mut cache)
        })
        .max()
        .unwrap();

    cache.insert((digits, n), max);
    max
}

fn main() {
    let banks = input::parse_lines(
        any_char()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .repeat(..),
    );

    for i in [2, 12] {
        let sol = banks
            .par_iter()
            .map(|v| joltage(v, i, &mut HashMap::new()))
            .sum::<usize>();
        println!("{sol}");
    }
}
