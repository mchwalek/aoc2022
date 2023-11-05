mod cli_parser;
mod command_parser;
mod file_system;

use std::{fs::File, io::{self, BufRead}};

use self::cli_parser::CliParser;

fn run(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines_iter = reader.lines().map(|x| x.unwrap()).peekable();

    let fs = CliParser::parse(&mut lines_iter).unwrap();
    fs.depth_first_dirs_iter()
        .map(|x| fs.dir_size(x))
        .filter(|x| *x <= 100000)
        .sum()
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