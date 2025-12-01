use aoc_utils::{input, parser::*};

fn password(rotations: &[i32]) -> usize {
    let mut start = 50;
    let mut cnt = 0;
    for i in rotations {
        start = (start + i) % 100;
        if start == 0 {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let rotations = input::parse_lines(
        prefix("R") >> number().map(|n| n as i32) | prefix("L") >> number().map(|n| -(n as i32)),
    );

    let part1 = password(&rotations);
    println!("{part1}");

    let rotations = rotations
        .iter()
        .flat_map(|x| vec![x.signum(); x.unsigned_abs() as usize])
        .collect::<Vec<_>>();

    let part2 = password(&rotations);
    println!("{part2}");
}
