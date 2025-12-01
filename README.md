# Advent of code v2

My solutions to the 12 problem version of advent of code in rust. Solutions from previous years made in Haskell can be found ![here](https://github.com/javimanotas/advent-of-code).

## Running

A makefile is provided.

To create the necessary files for implementing a solution run:
```bash
make setup YEAR=<year> DAY=<day>
```

To execute a solution run:
```bash
make run YEAR=<year> DAY=<day>
```

When running a solution, input files are expected to be found in the root in `/inputs/<year>/<day>`.
They are not uploaded to the repository because the creator requests it.

## Utils

Reading and parsing input in advent of code can be very tedious and repetitive.

This workspace provides a well documented package (`aoc_utils`) with common functionality that solves this issue.
