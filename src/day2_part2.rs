use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq, Debug, Clone, Copy)]
enum RPSShape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for RPSShape {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(format!("Unsupported value: {}", value)),
        }
    }
}

impl RPSShape {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(PartialEq, Debug)]
enum RPSResult {
    Win,
    Loss,
    Draw,
}

impl TryFrom<&str> for RPSResult {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(format!("Unsupported value: {}", value)),
        }
    }
}

impl RPSResult {
    fn counter_pick(&self, opponent_pick: RPSShape) -> RPSShape {
        match (self, opponent_pick) {
            (Self::Draw, pick) => pick,
            (RPSResult::Win, RPSShape::Rock) => RPSShape::Paper,
            (RPSResult::Win, RPSShape::Paper) => RPSShape::Scissors,
            (RPSResult::Win, RPSShape::Scissors) => RPSShape::Rock,
            (RPSResult::Loss, RPSShape::Rock) => RPSShape::Scissors,
            (RPSResult::Loss, RPSShape::Paper) => RPSShape::Rock,
            (RPSResult::Loss, RPSShape::Scissors) => RPSShape::Paper,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

pub fn run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        sum += line_score(line);
    }

    sum
}

fn line_score(line: String) -> i32 {
    let opponent_pick = RPSShape::try_from(&line[0..1]).unwrap();
    let expected_fight_result = RPSResult::try_from(&line[2..3]).unwrap();

    let my_pick = expected_fight_result.counter_pick(opponent_pick);

    my_pick.score() + expected_fight_result.score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_proper_enum() {
        assert_eq!(Ok(RPSShape::Rock), RPSShape::try_from("A"));
        assert_eq!(Ok(RPSShape::Paper), RPSShape::try_from("B"));
        assert_eq!(Ok(RPSShape::Scissors), RPSShape::try_from("C"));

        assert_eq!(Ok(RPSResult::Loss), RPSResult::try_from("X"));
        assert_eq!(Ok(RPSResult::Draw), RPSResult::try_from("Y"));
        assert_eq!(Ok(RPSResult::Win), RPSResult::try_from("Z"));

        assert_eq!(
            Err(String::from("Unsupported value: invalid")),
            RPSShape::try_from("invalid")
        );
    }

    #[test]
    fn returns_proper_counter_pick() {
        assert_eq!(RPSShape::Paper, RPSResult::Win.counter_pick(RPSShape::Rock));
        assert_eq!(
            RPSShape::Scissors,
            RPSResult::Win.counter_pick(RPSShape::Paper)
        );
        assert_eq!(
            RPSShape::Rock,
            RPSResult::Win.counter_pick(RPSShape::Scissors)
        );

        assert_eq!(RPSShape::Rock, RPSResult::Draw.counter_pick(RPSShape::Rock));
        assert_eq!(
            RPSShape::Paper,
            RPSResult::Draw.counter_pick(RPSShape::Paper)
        );
        assert_eq!(
            RPSShape::Scissors,
            RPSResult::Draw.counter_pick(RPSShape::Scissors)
        );

        assert_eq!(
            RPSShape::Scissors,
            RPSResult::Loss.counter_pick(RPSShape::Rock)
        );
        assert_eq!(
            RPSShape::Rock,
            RPSResult::Loss.counter_pick(RPSShape::Paper)
        );
        assert_eq!(
            RPSShape::Paper,
            RPSResult::Loss.counter_pick(RPSShape::Scissors)
        );
    }

    #[test]
    fn returns_proper_score() {
        assert_eq!(8, line_score(String::from("A Z"))); // win against rock (paper) => 2 (paper) + 6 (win)
        assert_eq!(4, line_score(String::from("A Y"))); // draw against rock (rock) => 1 (rock) + 3 (draw)
        assert_eq!(3, line_score(String::from("A X"))); // loss against rock (scissors) => 3 (scissors) + 0 (loss)
    }

    #[test]
    fn returns_answer() {
        let result = run("inputs/day2.txt");
        println!("{}", result);
    }
}
