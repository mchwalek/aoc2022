mod cli_parser;
mod command_parser;
mod file_system;

use std::{fs::File, io::{self, BufRead}};

fn run(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let lines_iter = reader.lines();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_answer() {
        let result = run("inputs/day7.txt");
        println!("{}", result);
    }
}