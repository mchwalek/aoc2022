use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(PartialEq, Debug)]
struct Group {
    first_rucksack: Rucksack,
    second_rucksack: Rucksack,
    third_rucksack: Rucksack,
}

impl Group {
    fn new(bundle: [String; 3]) -> Group {
        Group {
            first_rucksack: Rucksack::new(&bundle[0]),
            second_rucksack: Rucksack::new(&bundle[1]),
            third_rucksack: Rucksack::new(&bundle[2]),
        }
    }

    fn find_badge(self) -> Option<Item> {
        let first_rucksack_items = self.first_rucksack.all_items();
        let second_rucksack_items = self.second_rucksack.all_items();
        let third_rucksack_items = self.third_rucksack.all_items();

        let mut matched_index = None;
        for (i, item) in first_rucksack_items.iter().enumerate() {
            if second_rucksack_items.contains(item) && third_rucksack_items.contains(item) {
                matched_index = Some(i);
                break;
            }
        }

        matched_index.and_then(|i| first_rucksack_items.into_iter().nth(i))
    }
}

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
            second_compartment: Compartment::new(&items_string[half_length..]),
        }
    }

    fn all_items(self) -> Vec<Item> {
        self.first_compartment
            .items
            .into_iter()
            .chain(self.second_compartment.items)
            .collect()
    }
}

#[derive(PartialEq, Debug)]
struct Compartment {
    items: Vec<Item>,
}

impl Compartment {
    fn new(items_string: &str) -> Compartment {
        let items: Vec<_> = items_string.chars().map(Item).collect();

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

pub fn run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut sum = 0;

    let mut line_iter = reader.lines();
    'outer: loop {
        let mut bundle: [String; 3] = Default::default();

        for item in bundle.iter_mut() {
            if let Some(result) = line_iter.next() {
                item.insert_str(0, &result.unwrap());
            } else {
                break 'outer;
            }
        }

        let group = Group::new(bundle);
        let badge = group.find_badge();
        sum += badge.unwrap().priority();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_group() {
        let expected_group = Group {
            first_rucksack: Rucksack {
                first_compartment: Compartment {
                    items: vec![Item('a'), Item('b')],
                },
                second_compartment: Compartment {
                    items: vec![Item('c'), Item('d')],
                },
            },
            second_rucksack: Rucksack {
                first_compartment: Compartment {
                    items: vec![Item('e')],
                },
                second_compartment: Compartment {
                    items: vec![Item('f')],
                },
            },
            third_rucksack: Rucksack {
                first_compartment: Compartment {
                    items: vec![Item('g')],
                },
                second_compartment: Compartment {
                    items: vec![Item('h')],
                },
            },
        };

        let bundle: [String; 3] = [String::from("abcd"), String::from("ef"), String::from("gh")];
        assert_eq!(expected_group, Group::new(bundle));
    }

    #[test]
    fn returns_badge_if_present() {
        let duplicate_bundle = [
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
        ];
        let non_duplicate_bundle = [
            String::from("vJwpWtwJgWhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
        ];

        assert_eq!(Some(Item('r')), Group::new(duplicate_bundle).find_badge());
        assert_eq!(None, Group::new(non_duplicate_bundle).find_badge());
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
