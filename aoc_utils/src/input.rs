//! Utility functions for reading the input files.
//!
//! Input files are expected to be located in the `inputs/<year>/<day>` directory.

use crate::parser::Parser;

use std::{env, fs, path::Path};

/// Reads the content of the input file for the specified year and day.
///
/// # Panics
/// - if the command-line arguments for year and day are missing.
/// - if the input file cannot be opened.
///
/// # Returns
/// A `String` containing the content of the input file.
pub fn get_content() -> String {
    let mut args = env::args();
    args.next();

    let year = args.next().expect("Missing command-line argument for year");
    let day = args.next().expect("Missing command-line argument for day");
    let path = Path::new("inputs").join(year).join(day);

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("Can't open file: {}. {e}", path.display()))
}

/// Reads and parses the content of the input file for the specified year and day.
///
/// # Panics
/// - if the command-line arguments for year and day are missing.
/// - if the input file cannot be opened.
/// - if the parser fails
///
/// # Returns
/// The parsed value of type `T`.
pub fn parse_input<T>(parser: impl Parser<Output = T>) -> T {
    parser
        .run_parser(&get_content())
        .expect("Failed parsing input file")
}

/// Reads the lines of the input file for the specified year and day.
///  
/// # Panics
/// - if the command-line arguments for year and day are missing.
/// - if the input file cannot be opened.
///
/// # Returns
/// A `Vec<String>` where each element is a line from the input file.
pub fn get_lines() -> Vec<String> {
    get_content().lines().map(str::to_string).collect()
}

/// Reads and parses the lines of the input file for the specified year and day.
///
/// # Panics
/// - if the command-line arguments for year and day are missing.
/// - if the input file cannot be opened.
/// - if the parser fails
///
/// # Returns
/// A `Vec<T>` with the parsed result of each line.
pub fn parse_lines<T>(parser: impl Parser<Output = T>) -> Vec<T> {
    get_content()
        .lines()
        .map(|line| parser.run_parser(line).expect("Failed parsing input file"))
        .collect::<Vec<_>>()
}

/// Reads and parses the lines of the input file for the specified year and day.
/// If the parser fails for a line, that line will be discarded.
///
/// # Panics
/// - if the command-line arguments for year and day are missing.
/// - if the input file cannot be opened.
///
/// # Returns
/// A `Vec<T>` with the parsed result of each line that succeeded.
pub fn parse_ok_lines<T>(parser: impl Parser<Output = T>) -> Vec<T> {
    get_content()
        .lines()
        .flat_map(|line| parser.run_parser(line))
        .collect::<Vec<_>>()
}
