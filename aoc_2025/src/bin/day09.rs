use aoc_utils::{input, parser::*};

use std::ops::RangeInclusive;

fn area(a: (usize, usize), b: (usize, usize)) -> usize {
    let w = (a.0 as isize - b.0 as isize).abs() + 1;
    let h = (a.1 as isize - b.1 as isize).abs() + 1;
    (w * h) as usize
}

enum Edge {
    Horizontal {
        row: usize,
        colums: RangeInclusive<usize>,
    },
    Vertical {
        rows: RangeInclusive<usize>,
        column: usize,
    },
}

impl Edge {
    fn line_overlap(a: usize, b: usize, c: usize, d: usize) -> bool {
        let (_, x) = (a, b).min((c, d));
        let (y, _) = (a, b).max((c, d));
        x >= y
    }

    fn overlaps(&self, min_row: usize, max_row: usize, min_col: usize, max_col: usize) -> bool {
        match self {
            Edge::Horizontal { row, colums } => {
                *row >= min_row
                    && *row <= max_row
                    && Edge::line_overlap(*colums.start(), *colums.end(), min_col, max_col)
            }
            Edge::Vertical { rows, column } => {
                *column >= min_col
                    && *column <= max_col
                    && Edge::line_overlap(*rows.start(), *rows.end(), min_row, max_row)
            }
        }
    }
}

fn valid(a: (usize, usize), b: (usize, usize), edges: &[Edge]) -> bool {
    !edges.iter().any(|e| {
        e.overlaps(
            a.0.min(b.0) + 1,
            a.0.max(b.0) - 1,
            a.1.min(b.1) + 1,
            a.1.max(b.1) - 1,
        )
    })
}

fn main() {
    let coords = &input::parse_lines(number() + (prefix(",") >> number()));
    let part1 = (0..coords.len())
        .flat_map(|i| (i + 1..coords.len()).map(move |j| area(coords[i], coords[j])))
        .max()
        .unwrap();
    println!("{part1}");

    let edges = &(0..coords.len())
        .map(|i| {
            let a = coords[i];
            let b = coords[(i + 1) % coords.len()];
            if a.0 == b.0 {
                Edge::Horizontal {
                    row: a.0,
                    colums: a.1.min(b.1)..=a.1.max(b.1),
                }
            } else {
                Edge::Vertical {
                    rows: a.0.min(b.0)..=a.0.max(b.0),
                    column: a.1,
                }
            }
        })
        .collect::<Vec<_>>();
    let part2 = (0..coords.len())
        .flat_map(|i| {
            (i + 1..coords.len()).filter_map(move |j| {
                valid(coords[i], coords[j], edges).then_some(area(coords[i], coords[j]))
            })
        })
        .max()
        .unwrap();
    println!("{part2}");
}
