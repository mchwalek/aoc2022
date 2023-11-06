mod commands;
mod crate_stacks;
mod lib;

use std::{
    fs::File,
    io::{self, BufRead},
};

use self::{commands::Commands, crate_stacks::CrateStacks};

fn run_part1(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let line_iter = reader.lines();
    let (stack_lines, command_lines) = lib::split_lines(line_iter);

    let crate_stacks = CrateStacks::new(&stack_lines).unwrap();
    let commands = Commands::new(&command_lines).unwrap();
    let updated_stacks = crate_stacks.update(commands, lib::Order::LIFO).unwrap();

    updated_stacks.tops_string()
}

fn run_part2(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let line_iter = reader.lines();
    let (stack_lines, command_lines) = lib::split_lines(line_iter);

    let crate_stacks = CrateStacks::new(&stack_lines).unwrap();
    let commands = Commands::new(&command_lines).unwrap();
    let updated_stacks = crate_stacks.update(commands, lib::Order::FIFO).unwrap();

    updated_stacks.tops_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_answer_for_part1() {
        let result = run_part1("inputs/day5.txt");
        println!("{}", result);
    }

    #[test]
    fn returns_answer_for_part2() {
        let result = run_part2("inputs/day5.txt");
        println!("{}", result);
    }
}
