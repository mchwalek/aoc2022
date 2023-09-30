use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Commands<'a> {
    _storage: Vec<Command<'a>>
}

impl<'a> Commands<'a> {
    pub fn new(command_lines: &'a Vec<String>) -> Result<Commands<'a>, String> {
        let mut result = Commands { _storage: Vec::new() };

        for line in command_lines {

            result._storage.push(line.as_str().try_into()?);
        }

        Ok(result)
    }
}

#[derive(PartialEq, Debug)]
struct Command<'a> {
    _count: i32,
    _from: &'a str,
    _to: &'a str
}

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"^move (?P<count>.+) from (?P<from>.+) to (?P<to>.+)$").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = String;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let captures = COMMAND_REGEX.captures(s).ok_or(format!("couldn't parse command '{}'", s))?;
        let count_string = captures.name("count").unwrap().as_str();
        let count: i32 = count_string.parse().map_err(|_| format!("couldn't parse count '{}' in command '{}'", count_string, s))?;

        let from = captures.name("from").unwrap().as_str();
        if WHITESPACE_REGEX.is_match(from) {
            return Err(format!("couldn't parse from '{}' in command '{}'", from, s));
        }

        let to = captures.name("to").unwrap().as_str();
        if WHITESPACE_REGEX.is_match(to) {
            return Err(format!("couldn't parse to '{}' in command '{}'", to, s));
        }

        Ok(Command { _count: count, _from: from, _to: to })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_command_collection() {
        let expected_commands = vec![
            Command { _count: 3, _from: "8", _to: "9" },
            Command { _count: 2, _from: "2", _to: "8" },
        ];

        let command_lines = vec![
            "move 3 from 8 to 9".to_string(),
            "move 2 from 2 to 8".to_string()
        ];

        assert_eq!(Ok(Commands { _storage: expected_commands }), Commands::new(&command_lines));
    }

    #[test]
    fn returns_error_If_any_line_parse_fails() {
        let command_lines = vec![
            "move 3 from 8 to 9".to_string(),
            "invalid".to_string(),
            "move 2 from 2 to 8".to_string()
        ];

        assert_eq!(Err("couldn't parse command 'invalid'".to_string()), Commands::new(&command_lines));
    }

    #[test]
    fn handles_syntax_errors() {
        assert_eq!(Err::<Command, _>("couldn't parse command 'invalid'".to_string()), "invalid".try_into()); // completely invalid
        assert_eq!(Err::<Command, _>("couldn't parse command 'amove 3 from 8 to 9'".to_string()), "amove 3 from 8 to 9".try_into()); // leading chars forbidden
    }

    #[test]
    fn handles_count_errors() {
        assert_eq!(Err::<Command, _>("couldn't parse count ' ' in command 'move   from 8 to 9'".to_string()), "move   from 8 to 9".try_into());
        assert_eq!(Err::<Command, _>("couldn't parse count 'a' in command 'move a from 8 to 9'".to_string()), "move a from 8 to 9".try_into());
    }

    #[test]
    fn handles_from_identifier_errors() {
        assert_eq!(Err::<Command, _>("couldn't parse from ' ' in command 'move 3 from   to 9'".to_string()), "move 3 from   to 9".try_into());
        assert_eq!(Err::<Command, _>("couldn't parse from '8 ' in command 'move 3 from 8  to 9'".to_string()), "move 3 from 8  to 9".try_into());
    }

    #[test]
    fn handles_to_identifier_errors() {
        assert_eq!(Err::<Command, _>("couldn't parse to ' ' in command 'move 3 from 8 to  '".to_string()), "move 3 from 8 to  ".try_into());
        assert_eq!(Err::<Command, _>("couldn't parse to '9 ' in command 'move 3 from 8 to 9 '".to_string()), "move 3 from 8 to 9 ".try_into());
    }
}