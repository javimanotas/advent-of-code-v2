use std::path::Path;
use std::process::Command;
use std::{env, fs};
use toml_edit::DocumentMut;

fn create_crate(name: &str, path: &Path) {
    println!("Creating crate {}...", name);

    Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(name)
        .status()
        .expect("Failed to execute cargo new");

    fs::remove_file(path.join("src/lib.rs")).expect("Failed to remove lib.rs");
    fs::create_dir(path.join("src/bin")).expect("Failed to create bin directory");

    let toml_content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
aoc_utils = {{ path = "../aoc_utils" }}
itertools = "0.14.0"
"#,
        name
    );

    fs::write(path.join("Cargo.toml"), toml_content).expect("Failed to write to Cargo.toml");
}

fn add_crate_to_workspace(name: &str) {
    let toml_path = Path::new("Cargo.toml");

    let mut doc = fs::read_to_string(toml_path)
        .expect("Failed to read root Cargo.toml")
        .parse::<DocumentMut>()
        .expect("Failed to parse root Cargo.toml");

    doc.get_mut("workspace")
        .and_then(|ws| ws.get_mut("members"))
        .and_then(|m| m.as_array_mut())
        .map(|m| m.push(name));

    fs::write(toml_path, doc.to_string()).expect("Failed to write to root Cargo.toml");
}

fn setup_crate(crate_name: &str) {
    let crate_path = Path::new(&crate_name);

    if !crate_path.exists() {
        create_crate(crate_name, crate_path);
        add_crate_to_workspace(crate_name);
    }
}

fn setup_solution(crate_name: &str, day: &str) {
    let day_file_path = Path::new(&crate_name)
        .join("src/bin")
        .join(format!("day{}.rs", day));

    if day_file_path.exists() {
        println!("Day file {:?} already exists.", day_file_path);
    } else {
        println!("Creating day file {:?}...", day_file_path);
        let day_template = r#"use aoc_utils::{input, parser::*};

fn main() {
    // let part1 = todo!();
    // println!("{part1}");

    // let part2 = todo!();
    // println!("{part2}");
}
"#;
        fs::write(&day_file_path, day_template).expect("Failed to create day file");
    }
}

fn setup_input(year: &str, day: &str) {
    let dir = Path::new("inputs").join(year);
    fs::create_dir_all(&dir).expect("Failed to create input directory");

    let file = dir.join(day);

    if file.exists() {
        println!("Input file {:?} already exists.", file);
    } else {
        println!("Creating input file {:?}...", file);
        fs::File::create(&file).expect("Failed to create input file");
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    let year = args.next().expect("Missing command-line argument for year");
    let day = args.next().expect("Missing command-line argument for day");

    let crate_name = format!("aoc_{}", year);

    setup_crate(&crate_name);
    setup_solution(&crate_name, &day);
    setup_input(&year, &day);

    println!("Done!");
}
