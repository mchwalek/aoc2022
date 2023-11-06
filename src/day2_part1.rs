use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq, Debug)]
enum RPSShape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for RPSShape {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("Unsupported value: {}", value)),
        }
    }
}

impl RPSShape {
    fn fight(&self, other: &Self) -> RPSResult {
        match (self, other) {
            (Self::Rock, Self::Scissors) => RPSResult::Win,
            (Self::Rock, Self::Paper) => RPSResult::Loss,
            (Self::Paper, Self::Rock) => RPSResult::Win,
            (Self::Paper, Self::Scissors) => RPSResult::Loss,
            (Self::Scissors, Self::Paper) => RPSResult::Win,
            (Self::Scissors, Self::Rock) => RPSResult::Loss,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => RPSResult::Draw,
        }
    }

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

impl RPSResult {
    fn score(&self) -> i32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

fn run(path: &str) -> i32 {
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
    let my_pick = RPSShape::try_from(&line[2..3]).unwrap();

    let fight_result = my_pick.fight(&opponent_pick);
    my_pick.score() + fight_result.score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_proper_enum() {
        assert_eq!(Ok(RPSShape::Rock), RPSShape::try_from("A"));
        assert_eq!(Ok(RPSShape::Rock), RPSShape::try_from("X"));

        assert_eq!(Ok(RPSShape::Paper), RPSShape::try_from("B"));
        assert_eq!(Ok(RPSShape::Paper), RPSShape::try_from("Y"));

        assert_eq!(Ok(RPSShape::Scissors), RPSShape::try_from("C"));
        assert_eq!(Ok(RPSShape::Scissors), RPSShape::try_from("Z"));

        assert_eq!(
            Err(String::from("Unsupported value: invalid")),
            RPSShape::try_from("invalid")
        );
    }

    #[test]
    fn returns_proper_fight_result() {
        assert_eq!(RPSResult::Win, RPSShape::Rock.fight(&RPSShape::Scissors));
        assert_eq!(RPSResult::Loss, RPSShape::Rock.fight(&RPSShape::Paper));
        assert_eq!(RPSResult::Draw, RPSShape::Rock.fight(&RPSShape::Rock));

        assert_eq!(RPSResult::Win, RPSShape::Paper.fight(&RPSShape::Rock));
        assert_eq!(RPSResult::Loss, RPSShape::Paper.fight(&RPSShape::Scissors));
        assert_eq!(RPSResult::Draw, RPSShape::Paper.fight(&RPSShape::Paper));

        assert_eq!(RPSResult::Win, RPSShape::Scissors.fight(&RPSShape::Paper));
        assert_eq!(RPSResult::Loss, RPSShape::Scissors.fight(&RPSShape::Rock));
        assert_eq!(
            RPSResult::Draw,
            RPSShape::Scissors.fight(&RPSShape::Scissors)
        );
    }

    #[test]
    fn returns_proper_score() {
        assert_eq!(3, line_score(String::from("A Z"))); // rock vs scissors => 3 (scissors) + 0 (loss)
        assert_eq!(8, line_score(String::from("A Y"))); // rock vs paper => 2 (paper) + 6 (win)
        assert_eq!(4, line_score(String::from("A X"))); // rock vs rock => 1 (rock) + 3 (draw)
    }

    #[test]
    fn returns_answer() {
        let result = run("inputs/day2.txt");
        println!("{}", result);
    }
}
