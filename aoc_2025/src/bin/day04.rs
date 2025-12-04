use aoc_utils::{input, parser::*};

fn is_valid_roll(rolls: &[Vec<bool>], r: usize, c: usize) -> bool {
    (-1_isize..=1)
        .flat_map(|dr| {
            (-1_isize..=1).filter_map(move |dc| {
                (dr != 0 || dc != 0).then_some(())?;
                let row = rolls.get((r as isize + dr) as usize)?;
                let x = row.get((c as isize + dc) as usize)?;
                x.then_some(())
            })
        })
        .count()
        < 4
}

fn valid_rolls(rolls: &[Vec<bool>]) -> Vec<(usize, usize)> {
    (0..rolls.len())
        .flat_map(|r| {
            (0..rolls[r].len())
                .filter_map(move |c| (rolls[r][c] && is_valid_roll(rolls, r, c)).then_some((r, c)))
        })
        .collect()
}

fn remove_valids(rolls: &mut Vec<Vec<bool>>) -> usize {
    let mut v = valid_rolls(&rolls);
    let mut count = 0;

    while !v.is_empty() {
        count += v.len();

        for (r, c) in v {
            rolls[r][c] = false;
        }

        v = valid_rolls(&rolls);
    }

    count
}

fn main() {
    let mut rolls = input::parse_lines(any_char().map(|c| c == '@').repeat(..));

    let part1 = valid_rolls(&rolls).len();
    println!("{part1}");

    let part2 = remove_valids(&mut rolls);
    println!("{part2}");
}
