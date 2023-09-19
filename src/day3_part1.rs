use std::{fs::File, io::{self, BufRead}};

#[derive(PartialEq, Debug)]
struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Rucksack {
    fn new(items_string: &str) -> Rucksack {
        let half_length = items_string.len() / 2;

        Rucksack {
            first_compartment: Compartment::new(&items_string[..half_length]),
            second_compartment: Compartment::new(&items_string[half_length..])
        }
    }

    fn find_duplicate_item(self) -> Option<Item> {
        let mut matched_index = None;
        for (i, item) in self.first_compartment.items.iter().enumerate() {
            if self.second_compartment.items.contains(item) {
                matched_index = Some(i);
                break;
            }
        }

        matched_index.and_then(|i| self.first_compartment.items.into_iter().nth(i))
    }
}

#[derive(PartialEq, Debug)]
struct Compartment {
    items: Vec<Item>

}

impl Compartment {
    fn new(items_string: &str) -> Compartment {
        let items: Vec<_> = items_string.chars().map(|c| Item(c)).collect();

        Compartment { items }
    }
}

#[derive(PartialEq, Debug)]
struct Item(char);

impl Item {
    fn priority(self) -> i32 {
        match self.0 {
            c @ 'a'..='z' => c as i32 - 'a' as i32 + 1,
            c @ 'A'..='Z' => c as i32 - 'A' as i32 + 27,
            c => panic!("Unsupported character: {}", c),
        }
    }
}

fn run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();

        let rucksack = Rucksack::new(&line);
        let duplicate = rucksack.find_duplicate_item();
        sum += duplicate.unwrap().priority();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_rucksack() {
        let expected_rucksack = Rucksack {
            first_compartment: Compartment {
                items: vec![Item('a'), Item('b')]
            },
            second_compartment: Compartment {
                items: vec![Item('c'), Item('d')]
            }
        };

        assert_eq!(expected_rucksack, Rucksack::new("abcd"));
    }

    #[test]
    fn returns_duplicate_item_if_present() {
        let duplicate_rucksack = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        let non_duplicate_rucksack = Rucksack::new("vJrwWtwJgWrhcsFMMfFFhFp");

        assert_eq!(Some(Item('p')), duplicate_rucksack.find_duplicate_item());
        assert_eq!(None, non_duplicate_rucksack.find_duplicate_item());
    }

    #[test]
    fn returns_item_priority() {
        assert_eq!(16, Item('p').priority());
        assert_eq!(19, Item('s').priority());
        assert_eq!(20, Item('t').priority());
        assert_eq!(22, Item('v').priority());
        assert_eq!(38, Item('L').priority());
        assert_eq!(42, Item('P').priority());
    }

    #[test]
    fn returns_answer() {
        let result = run("inputs/day3.txt");
        println!("{}", result);
    }
}