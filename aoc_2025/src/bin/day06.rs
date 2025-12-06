use aoc_utils::{input, parser::*};

fn total(numbers: &[Vec<usize>], ops: &[&str]) -> usize {
    numbers
        .iter()
        .zip(ops)
        .map(|(v, op)| match *op {
            "+" => v.iter().sum::<usize>(),
            "*" => v.iter().product(),
            _ => panic!(),
        })
        .sum::<usize>()
}

fn column_numbes(digits_grid: &[Vec<char>], indexes: &[usize]) -> Vec<Vec<usize>> {
    indexes
        .iter()
        .map(|&i| {
            let mut digits = vec![];
            for i in (0..=i).rev() {
                let number = (0..digits_grid.len())
                    .filter_map(|c| (digits_grid[c][i] != ' ').then_some(digits_grid[c][i]))
                    .map(|c| c.to_string())
                    .collect::<String>()
                    .parse::<usize>();

                if let Ok(n) = number {
                    digits.push(n);
                } else {
                    break;
                }
            }
            digits
        })
        .collect()
}

fn main() {
    let numbers = input::parse_ok_lines(number().anywhere().repeat(1..));
    let ops = input::parse_ok_lines((prefix("*") | prefix("+")).anywhere().repeat(1..));
    let ops = &ops[0];

    let numbers = (0..numbers[0].len())
        .map(|i| {
            (0..numbers.len())
                .map(|j| numbers[j][i])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = total(&numbers, ops);
    println!("{part1}");

    let ops_chars = input::parse_ok_lines(
        prefix("+").anywhere().look_ahead() >> satisfies(|c| " +*".contains(c)).repeat(1..),
    );

    let ops_indexes = ops_chars[0]
        .iter()
        .enumerate()
        .flat_map(|(i, c)| (*c != ' ').then_some(i))
        .skip(1)
        .map(|n| n - 2)
        .chain([ops_chars[0].len() - 1])
        .collect::<Vec<_>>();

    let digits_grid = input::parse_ok_lines(
        satisfies(|c| c.is_ascii_digit() || c == ' ').repeat(ops_indexes[1]..),
    );

    let part2 = total(&column_numbes(&digits_grid, &ops_indexes), ops);
    println!("{part2}");
}
