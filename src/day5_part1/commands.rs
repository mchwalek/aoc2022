use std::vec::IntoIter;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Commands<'a> {
    storage: Vec<Command<'a>>
}

impl<'a> Commands<'a> {
    pub fn new(command_lines: &'a Vec<String>) -> Result<Commands<'a>, String> {
        let mut result = Commands { storage: Vec::new() };

        for line in command_lines {

            result.storage.push(line.as_str().try_into()?);
        }

        Ok(result)
    }
}

impl<'a> IntoIterator for Commands<'a> {
    type Item = Command<'a>;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage.into_iter()
    }
}

#[derive(PartialEq, Debug)]
pub struct Command<'a> {
    pub count: u32,
    pub from: &'a str,
    pub to: &'a str
}

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"^move (?P<count>.+) from (?P<from>.+) to (?P<to>.+)$").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let captures = COMMAND_REGEX.captures(s).ok_or(format!("invalid command '{}'", s))?;
        let count_string = captures.name("count").unwrap().as_str();
        let count: i32 = count_string.parse().map_err(|_| format!("invalid count '{}' in command '{}'", count_string, s))?;
        if count <= 0 {
            return Err(format!("nonpositive count '{}' in command '{}'", count_string, s));
        }

        let from = captures.name("from").unwrap().as_str();
        if WHITESPACE_REGEX.is_match(from) {
            return Err(format!("invalid from '{}' in command '{}'", from, s));
        }

        let to = captures.name("to").unwrap().as_str();
        if WHITESPACE_REGEX.is_match(to) {
            return Err(format!("invalid to '{}' in command '{}'", to, s));
        }

        Ok(Command { count: count as u32, from, to })
    }
}

#[cfg(test)]
mod tests {
    mod commands {
        use crate::day5_part1::commands::*;

        #[test]
        fn initializes_command_collection() {
            let expected_commands = vec![
                Command { count: 3, from: "8", to: "9" },
                Command { count: 2, from: "2", to: "8" },
            ];

            let command_lines = vec![
                "move 3 from 8 to 9".to_string(),
                "move 2 from 2 to 8".to_string()
            ];

            assert_eq!(Ok(Commands { storage: expected_commands }), Commands::new(&command_lines));
        }

        #[test]
        fn returns_error_if_any_line_parse_fails() {
            let command_lines = vec![
                "move 3 from 8 to 9".to_string(),
                "invalid".to_string(),
                "move 2 from 2 to 8".to_string()
            ];

            assert_eq!(Err("invalid command 'invalid'".to_string()), Commands::new(&command_lines));
        }
    }

    mod command {
        use crate::day5_part1::commands::*;

        #[test]
        fn handles_syntax_errors() {
            assert_eq!(Err::<Command, _>("invalid command 'invalid'".to_string()), "invalid".try_into()); // completely invalid
            assert_eq!(Err::<Command, _>("invalid command 'amove 3 from 8 to 9'".to_string()), "amove 3 from 8 to 9".try_into()); // leading chars forbidden
        }

        #[test]
        fn handles_count_errors() {
            assert_eq!(Err::<Command, _>("invalid count ' ' in command 'move   from 8 to 9'".to_string()), "move   from 8 to 9".try_into());
            assert_eq!(Err::<Command, _>("invalid count 'a' in command 'move a from 8 to 9'".to_string()), "move a from 8 to 9".try_into());
        }

        #[test]
        fn handles_nonpositive_count_errors() {
            assert_eq!(Err::<Command, _>("nonpositive count '0' in command 'move 0 from 8 to 9'".to_string()), "move 0 from 8 to 9".try_into());
            assert_eq!(Err::<Command, _>("nonpositive count '-1' in command 'move -1 from 8 to 9'".to_string()), "move -1 from 8 to 9".try_into());
        }

        #[test]
        fn handles_from_identifier_errors() {
            assert_eq!(Err::<Command, _>("invalid from ' ' in command 'move 3 from   to 9'".to_string()), "move 3 from   to 9".try_into());
            assert_eq!(Err::<Command, _>("invalid from '8 ' in command 'move 3 from 8  to 9'".to_string()), "move 3 from 8  to 9".try_into());
        }

        #[test]
        fn handles_to_identifier_errors() {
            assert_eq!(Err::<Command, _>("invalid to ' ' in command 'move 3 from 8 to  '".to_string()), "move 3 from 8 to  ".try_into());
            assert_eq!(Err::<Command, _>("invalid to '9 ' in command 'move 3 from 8 to 9 '".to_string()), "move 3 from 8 to 9 ".try_into());
        }
    }
}