use itertools::Itertools;

use super::lib::Slideable;

const PACKET_MARKER_SIZE: usize = 4;
const MESSAGE_MARKER_SIZE: usize = 14;

pub struct DataReader {
    data: String
}

impl DataReader {
    pub fn new(data: String) -> Self {
        DataReader { data }
    }

    pub fn find_start_of_packet(&self) -> Option<StartOfPacket> {
        self.find_start_base(PACKET_MARKER_SIZE).map(|x| StartOfPacket { start_base: x })
    }

    pub fn find_start_of_message(&self) -> Option<StartOfMessage> {
        self.find_start_base(MESSAGE_MARKER_SIZE).map(|x| StartOfMessage { start_base: x })
    }

    fn find_start_base(&self, marker_size: usize) -> Option<StartBase> {
        for (i, window) in self.data.sliding_window_iter(marker_size).enumerate() {
            if window.chars().duplicates().next().is_none() {
                return Some(StartBase { chars_processed: i + marker_size });
            }
        }

        None
    }
}

#[derive(PartialEq, Debug)]
struct StartBase {
    chars_processed: usize
}

#[derive(PartialEq, Debug)]
pub struct StartOfPacket {
    start_base: StartBase
}

impl StartOfPacket {
    pub fn get_chars_processed(&self) -> usize {
        self.start_base.chars_processed
    }
}

#[derive(PartialEq, Debug)]
pub struct StartOfMessage {
    start_base: StartBase
}

impl StartOfMessage {
    pub fn get_chars_processed(&self) -> usize {
        self.start_base.chars_processed
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::data_reader::*;

    #[test]
    fn returns_start_of_packet_if_found() {
        let reader = DataReader::new("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
        assert_eq!(Some(5), reader.find_start_of_packet().map(|x| x.get_chars_processed()));

        let reader2 = DataReader::new("bvwb".to_string());
        assert_eq!(None, reader2.find_start_of_packet());

        let reader3 = DataReader::new("bvw".to_string());
        assert_eq!(None, reader3.find_start_of_packet());
    }

    #[test]
    fn returns_start_of_message_if_found() {
        let reader = DataReader::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
        assert_eq!(Some(19), reader.find_start_of_message().map(|x| x.get_chars_processed()));

        let reader2 = DataReader::new("mjqjpqmgbljsph".to_string());
        assert_eq!(None, reader2.find_start_of_message());

        let reader3 = DataReader::new("bvw".to_string());
        assert_eq!(None, reader3.find_start_of_message());
    }
}