mod cli_parser;
mod command_parser;
mod file_system;

use std::{fs::File, io::{self, BufRead}};

use self::cli_parser::CliParser;

fn run(path: &str) -> (usize, usize) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines_iter = reader.lines().map(|x| x.unwrap()).peekable();

    let fs = CliParser::parse(&mut lines_iter).unwrap();
    let small_dirs_sum = fs.depth_first_dirs_iter()
        .map(|x| fs.dir_size(x))
        .filter(|x| *x <= 100000)
        .sum();

    let root_dir = fs.dirs_iter().next().unwrap();
    let free_space = 70000000 - fs.dir_size(root_dir);
    let space_needed = 30000000 - free_space;
    let smallest_dir_to_remove = fs.depth_first_dirs_iter()
        .map(|x| fs.dir_size(x))
        .filter(|x| *x >= space_needed)
        .min().unwrap();

    (small_dirs_sum, smallest_dir_to_remove)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_answer() {
        let result = run("inputs/day7.txt");
        println!("{:?}", result);
    }
}