mod lib;
mod start_of_packet_detector;

use std::{fs::File, io::{self, Read}};

use self::start_of_packet_detector::StartOfPacketDetector;

fn run(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut data = String::new();

    reader.read_to_string(&mut data).unwrap();
    StartOfPacketDetector.find(data).unwrap().get_chars_processed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_answer() {
        let result = run("inputs/day6.txt");
        println!("{}", result);
    }
}