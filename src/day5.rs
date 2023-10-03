mod crate_stacks;
mod commands;
mod lib;

use std::{fs::File, io::{self, BufRead}};

use self::{crate_stacks::CrateStacks, commands::Commands};

fn run(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let line_iter = reader.lines();
    let (stack_lines, command_lines) = lib::split_lines(line_iter);

    let crate_stacks = CrateStacks::new(&stack_lines).unwrap();
    let commands = Commands::new(&command_lines).unwrap();
    let updated_stacks = crate_stacks.update(commands).unwrap();

    updated_stacks.tops_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_answer() {
        let result = run("inputs/day5.txt");
        println!("{}", result);
    }
}