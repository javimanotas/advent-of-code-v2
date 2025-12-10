use aoc_utils::{input, parser::*};

fn toggle(state: &mut [bool], idx: &[usize]) {
    for &i in idx {
        state[i] = !state[i];
    }
}

fn reachable_with(
    state: &mut [bool],
    depth: usize,
    lights: &[bool],
    buttons: &[Vec<usize>],
) -> bool {
    if depth == 0 {
        return state == lights;
    }

    buttons.iter().any(|b| {
        toggle(state, b);
        let valid = reachable_with(state, depth - 1, lights, buttons);
        toggle(state, b);
        valid
    })
}

fn fewest_presses(counters: &[bool], buttons: &[Vec<usize>]) -> usize {
    (0..)
        .find(|i| reachable_with(&mut vec![false; counters.len()], *i, counters, buttons))
        .unwrap()
}

fn reachable_with_2(
    state: &mut [usize],
    idx: usize,
    presses: usize,
    counters: &[usize],
    buttons: &[Vec<usize>],
) -> bool {
    if presses == 0 {
        return state == counters;
    }

    if idx >= buttons.len() {
        return false;
    }

    for i in 0..=presses {
        for (n, j) in buttons[idx].iter().enumerate() {
            state[*j] += i;

            if state[*j] > counters[*j] {
                for k in 0..=n {
                    state[buttons[idx][k]] -= i;
                }
                return false;
            }
        }

        if reachable_with_2(state, idx + 1, presses - i, counters, buttons) {
            return true;
        }
        for j in &buttons[idx] {
            state[*j] -= i;
        }
    }

    return false;
}

fn fewest_presses_2(counters: &[usize], buttons: &[Vec<usize>]) -> usize {
    (*counters.iter().max().unwrap()..)
        .find(|i| reachable_with_2(&mut vec![0; counters.len()], 0, *i, counters, buttons))
        .unwrap()
}

fn main() {
    let lights = prefix("[")
        >> (prefix("#").map(|_| true) | prefix(".").map(|_| false)).repeat(1..)
        << prefix("]");
    let buttons = prefix(" (") >> number().sep_by(prefix(","), 1..) << prefix(")");
    let counters = prefix(" {") >> number().sep_by(prefix(","), 1..) << prefix("}");
    let machines = input::parse_lines(lights + buttons.repeat(1..) + counters);

    let part1 = machines
        .iter()
        .map(|((l, b), _)| fewest_presses(l, b))
        .sum::<usize>();
    println!("{part1}");

    // This takes forever so i dont't know if it gives the right answer.
    // Maybe later i try optimizing it or trying a different approach.
    let part2 = machines
        .iter()
        .map(|((_, b), c)| fewest_presses_2(c, b))
        .sum::<usize>();
    println!("{part2}");
}
