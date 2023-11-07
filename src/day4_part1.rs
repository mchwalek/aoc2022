use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq, Debug)]
struct Pair {
    first_assignment: Assignment,
    second_assignment: Assignment,
}

impl Pair {
    fn new(pair_string: &str) -> Pair {
        let (first_assignment_string, second_assignment_string) =
            pair_string.split_once(',').unwrap();

        Pair {
            first_assignment: Assignment::new(first_assignment_string),
            second_assignment: Assignment::new(second_assignment_string),
        }
    }

    fn has_contained_assignment(self) -> bool {
        self.first_assignment.contains(&self.second_assignment)
            || self.second_assignment.contains(&self.first_assignment)
    }
}

#[derive(PartialEq, Debug)]
struct Assignment {
    lower_bound: i32,
    upper_bound: i32,
}

impl Assignment {
    fn new(assignment_string: &str) -> Assignment {
        let (lower_bound_string, upper_bound_string) = assignment_string.split_once('-').unwrap();

        Assignment {
            lower_bound: lower_bound_string.parse().unwrap(),
            upper_bound: upper_bound_string.parse().unwrap(),
        }
    }

    fn contains(&self, other: &Assignment) -> bool {
        self.lower_bound <= other.lower_bound && self.upper_bound >= other.upper_bound
    }
}

pub fn run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut count = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();

        let pair = Pair::new(&line);
        if pair.has_contained_assignment() {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_pair() {
        let expected_pair = Pair {
            first_assignment: Assignment {
                lower_bound: 2,
                upper_bound: 4,
            },
            second_assignment: Assignment {
                lower_bound: 6,
                upper_bound: 8,
            },
        };
        assert_eq!(expected_pair, Pair::new("2-4,6-8"));
    }

    #[test]
    fn checks_if_has_contained_assignment() {
        assert!(Pair::new("1-3,2-2").has_contained_assignment());
        assert!(Pair::new("2-2,1-3").has_contained_assignment());
        assert!(Pair::new("1-3,1-3").has_contained_assignment());

        assert!(!Pair::new("1-3,2-4").has_contained_assignment());
        assert!(!Pair::new("1-2,4-5").has_contained_assignment());
    }

    #[test]
    fn returns_answer() {
        let result = run("inputs/day4.txt");
        println!("{}", result);
    }
}
