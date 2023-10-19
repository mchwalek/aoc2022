use std::io::Result;

use super::file_system::FileSystem;

pub struct CliParser {
}

impl CliParser {
    pub fn parse<'a, T: Iterator<Item = Result<String>>>(iterator: T) -> FileSystem {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::cli_parser::*;

    #[test]
    fn does_something() {
    }
}