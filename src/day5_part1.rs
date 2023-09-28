mod crate_stacks;
mod command_collection;
mod lib;

use std::{fs::File, io::{self, BufRead}};

use self::crate_stacks::CrateStacks;

fn run(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let line_iter = reader.lines();
    let (stack_lines, command_lines) = split_lines(line_iter);

    let crate_stacks = CrateStacks::new(&stack_lines);

    String::new()
}

fn split_lines<T: Iterator<Item  = io::Result<String>>>(mut iter: T) -> (Vec<String>, Vec<String>) {
    let mut stack_lines = Vec::new();
    loop {
        match iter.next() {
            Some(Ok(line)) if !line.is_empty() => stack_lines.push(line),
            _ => break,
        }
    }

    let mut command_lines = Vec::new();
    loop {
        match iter.next() {
            Some(Ok(line)) => command_lines.push(line),
            _ => break,
        }
    }

    (stack_lines, command_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_file_into_stack_and_command_lines() {
        let file_lines = vec![
            Ok("stack1".to_string()),
            Ok("stack2".to_string()),
            Ok("".to_string()),
            Ok("command1".to_string()),
            Ok("command2".to_string()),
        ];

        let (stack_lines, command_lines) = split_lines(file_lines.into_iter());
        let expected_stack_lines = vec!["stack1".to_string(), "stack2".to_string()];
        let expected_command_lines = vec!["command1".to_string(), "command2".to_string()];

        assert_eq!(expected_stack_lines, stack_lines);
        assert_eq!(expected_command_lines, command_lines);
    }

    #[test]
    fn returns_answer() {
        let result = run("inputs/day5.txt");
        println!("{}", result);
    }
}