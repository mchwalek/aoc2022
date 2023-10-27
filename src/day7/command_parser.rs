use std::iter::{self, Peekable};

use super::file_system::FileSystem;

pub struct CommandParser {
}

impl CommandParser {
    const COMMAND_LINE_PREFIX: &'static str = "$ ";

    pub fn parse<'a, 'b, T: Iterator<Item = String>>(iterator: &mut Peekable<T>) -> Result<Command, String> {
        let line = iterator.next().ok_or_else(|| "empty iterator passed".to_string())?;
        if !Self::command_line(&line) {
            return Err(format!("not a command line: '{}'", line));
        }

        let content = &line[Self::COMMAND_LINE_PREFIX.len()..];
        let parts: Vec<_> = content.split(' ').collect();
        let (name, args) = parts.split_first().unwrap();

        match *name {
            "cd" => {
                if args.len() != 1 {
                    return Err(format!("expected exactly 1 param in line '{}'", line));
                }

                Ok(Command::Cd { dir: args[0].to_string() })
            },
            "ls" => {
                let mut output: Vec<String> = Vec::new();
                while !Self::on_last_output_line(iterator) {
                    let output_line = iterator.next().unwrap();
                    output.push(output_line);
                }

                Ok(Command::Ls { output })
            },
            _ => Err(format!("invalid command '{}' in line '{}'", name, line)),
        }
    }

    fn command_line(line: &str) -> bool {
        line.starts_with(Self::COMMAND_LINE_PREFIX)
    }

    fn on_last_output_line<T: Iterator<Item = String>>(iterator: &mut Peekable<T>) -> bool {
        match iterator.peek() {
            Some(line) => Self::command_line(line),
            None => true,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Command {
    Cd { dir: String },
    Ls { output: Vec<String> },
}

impl Command {
    pub fn update_fs(&self, fs: &mut FileSystem) {
        match self {
            Command::Ls { output } => todo!(),
            Command::Cd { dir } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::command_parser::*;

    #[test]
    fn returns_cd_command() {
        let mut iter = command_string_to_iter("$ cd dir".to_string());
        assert_eq!(Ok(Command::Cd { dir: "dir".to_string() }), CommandParser::parse(&mut iter));
    }

    #[test]
    fn returns_ls_command() {
        let mut iter = vec![
            "$ ls".to_string(),
            "dir a".to_string(),
            "123 b.txt".to_string()
        ].into_iter().peekable();
        assert_eq!(
            Ok(Command::Ls { output: vec!["dir a".to_string(), "123 b.txt".to_string()] }),
            CommandParser::parse(&mut iter));
    }

    #[test]
    fn handles_general_errors() {
        let mut empty_iter = iter::empty::<String>().peekable();
        assert_eq!(Err("empty iterator passed".to_string()), CommandParser::parse(&mut empty_iter));

        let mut iter = command_string_to_iter("cd dir".to_string());
        assert_eq!(Err("not a command line: 'cd dir'".to_string()), CommandParser::parse(&mut iter));

        iter = command_string_to_iter("$ ".to_string());
        assert_eq!(Err("invalid command '' in line '$ '".to_string()), CommandParser::parse(&mut iter));

        iter = command_string_to_iter("$ cp a b".to_string());
        assert_eq!(Err("invalid command 'cp' in line '$ cp a b'".to_string()), CommandParser::parse(&mut iter));
    }

    #[test]
    fn handles_cd_command_errors() {
        let mut iter = command_string_to_iter("$ cd".to_string());
        assert_eq!(Err("expected exactly 1 param in line '$ cd'".to_string()), CommandParser::parse(&mut iter));

        iter = command_string_to_iter("$ cd dir dir2".to_string());
        assert_eq!(Err("expected exactly 1 param in line '$ cd dir dir2'".to_string()), CommandParser::parse(&mut iter));
    }

    fn command_string_to_iter(command: String) -> Peekable<impl Iterator<Item = String>> {
        vec![command].into_iter().peekable()
    }
}