use itertools::Itertools;

use super::lib::Slideable;

const MARKER_SIZE: usize = 4;

pub struct StartOfPacketDetector;

impl StartOfPacketDetector {
    pub fn find(&self, data: String) -> Option<StartOfPacket> {
        for (i, window) in data.sliding_window_iter(MARKER_SIZE).enumerate() {
            if window.chars().duplicates().next().is_none() {
                return Some(StartOfPacket { chars_processed: i + MARKER_SIZE })
            }
        }

        None
    }
}

#[derive(PartialEq, Debug)]
pub struct StartOfPacket {
    chars_processed: usize
}

impl StartOfPacket {
    pub fn get_chars_processed(&self) -> usize {
        self.chars_processed
    }
}

#[cfg(test)]
mod tests {
    use crate::day6_part1::start_of_packet_detector::*;

    #[test]
    fn returns_start_of_packet_if_found() {
        let detector = StartOfPacketDetector;
        assert_eq!(Some(StartOfPacket { chars_processed: 5 }), detector.find("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!(None, detector.find("bvwb".to_string()));
        assert_eq!(None, detector.find("bvw".to_string()));
    }
}