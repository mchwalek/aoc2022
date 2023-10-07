mod lib;
mod data_reader;

use std::{fs::File, io::{self, Read}};

use self::data_reader::DataReader;

fn run(path: &str) -> (usize, usize) {
    let file = File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut data = String::new();

    reader.read_to_string(&mut data).unwrap();
    let data_reader = DataReader::new(data);

    let start_of_packet = data_reader.find_start_of_packet().unwrap();
    let start_of_message = data_reader.find_start_of_message().unwrap();

    (start_of_packet.get_chars_processed(), start_of_message.get_chars_processed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_answer() {
        let result = run("inputs/day6.txt");
        println!("{:?}", result);
    }
}