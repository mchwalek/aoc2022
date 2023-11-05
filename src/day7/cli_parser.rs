use std::iter::Peekable;

use super::command_parser::CommandParser;
use super::file_system::FileSystem;

pub struct CliParser {
}

impl CliParser {
    pub fn parse<T: Iterator<Item = String>>(iterator: &mut Peekable<T>) -> Result<FileSystem, String> {
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
    use std::collections::HashSet;

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
        ].into_iter().peekable();

        let fs = CliParser::parse(&mut iter).unwrap();

        let mut expected_dirs = HashSet::new();
        expected_dirs.insert("/".to_string());
        expected_dirs.insert("/b".to_string());
        expected_dirs.insert("/b/b".to_string());
        expected_dirs.insert("/b/c".to_string());
        let result: HashSet<_> = fs.depth_first_dirs_iter().map(|x| fs.dir_path(x)).collect();
        assert_eq!(expected_dirs, result);

        let mut expected_files = HashSet::new();
        expected_files.insert("/a".to_string());
        expected_files.insert("/b/a".to_string());
        expected_files.insert("/b/b/a".to_string());
        expected_files.insert("/b/c/a".to_string());
        let result: HashSet<_> = fs.depth_first_files_iter().map(|x| fs.file_path(x)).collect();
        assert_eq!(expected_files, result);
    }
}