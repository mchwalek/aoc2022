use std::fmt::Write;
use std::iter::Peekable;

use super::command_parser::CommandParser;
use super::file_system::FileSystem;

pub struct CliParser {
}

impl CliParser {
    pub fn parse<T: Iterator<Item = std::io::Result<String>>>(iterator: &mut T) -> Result<FileSystem, String> {
        let iterator = &mut iterator.map(|x| x.unwrap()).peekable();

        let mut result = FileSystem::new();
        while !Self::on_last_line(iterator) {
            let command = CommandParser::parse(iterator)?;
            command.update_fs(&mut result)?;
        }

        Ok(result)
    }

    fn on_last_line<T: Iterator<Item = String>>(iterator: &mut Peekable<T>) -> bool {
        iterator.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::cli_parser::*;

    #[test]
    fn returns_expected_file_system() {
        let mut iter = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "12 a".to_string(),
            "dir b".to_string(),
            "$ cd b".to_string(),
            "$ ls".to_string(),
            "34 a".to_string(),
            "dir b".to_string(),
            "dir c".to_string(),
            "$ cd b".to_string(),
            "$ ls".to_string(),
            "56 a".to_string(),
            "$ cd ..".to_string(),
            "$ cd c".to_string(),
            "$ ls".to_string(),
            "78 a".to_string(),
        ].into_iter().map(|x| Ok(x)).peekable();

        let mut expected_representation = String::new();
        write!(expected_representation, "dir /\n").unwrap();
        write!(expected_representation, "  12 a\n").unwrap();
        write!(expected_representation, "  dir b\n").unwrap();
        write!(expected_representation, "    34 a\n").unwrap();
        write!(expected_representation, "    dir b\n").unwrap();
        write!(expected_representation, "      56 a\n").unwrap();
        write!(expected_representation, "    dir c\n").unwrap();
        write!(expected_representation, "      78 a\n").unwrap();

        let fs = CliParser::parse(&mut iter).unwrap();
        assert_eq!(expected_representation, fs.string_representation());
    }
}