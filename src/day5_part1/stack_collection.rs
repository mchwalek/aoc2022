use std::collections::HashMap;

use regex::Regex;

use super::lib::Stack;
use super::models::Crate;

#[derive(PartialEq, Debug)]
struct StackCollection<'a> {
    _storage: HashMap<&'a str, Stack<Crate>>,
}

impl<'a> StackCollection<'a> {
    fn new(stack_lines: &'a Vec<String>) -> StackCollection<'a> {

        let (id_line, content_lines) = stack_lines.split_last().unwrap();

        let id_regex = Regex::new(r"[^ ]").unwrap();
        let id_lookup: HashMap<_, _> = id_regex
            .find_iter(id_line)
            .map(|m| (m.as_str(), m.start()))
            .collect();

        let mut result = StackCollection {
            _storage: id_lookup
                .iter()
                .map(|(&id, _)| (id, Stack::new()))
                .collect()
        };

        for line in content_lines.into_iter().rev() {
            for (&id, &index) in id_lookup.iter() {
                if let Some(crate_char) = line.chars().nth(index) {
                    if crate_char != ' ' {
                        let stack = result._storage.get_mut(id).unwrap();
                        stack.push(Crate(crate_char));
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_stack_collection() {
        let mut expected_stacks = HashMap::<_, Stack<Crate>>::new();
        expected_stacks.insert("1", "PFMQWGRT".into());
        expected_stacks.insert("2", "HFR".into());
        expected_stacks.insert("3", "PZRVGHSD".into());
        expected_stacks.insert("4", "QHPBFWG".into());
        expected_stacks.insert("5", "PSMJH".into());
        expected_stacks.insert("6", "MZTHSRPL".into());
        expected_stacks.insert("7", "PTHNML".into());
        expected_stacks.insert("8", "FDQR".into());
        expected_stacks.insert("9", "DSCNLPH".into());

        let stack_lines = vec![
            "[T]     [D]         [L]            ".to_string(),
            "[R]     [S] [G]     [P]         [H]".to_string(),
            "[G]     [H] [W]     [R] [L]     [P]".to_string(),
            "[W]     [G] [F] [H] [S] [M]     [L]".to_string(),
            "[Q]     [V] [B] [J] [H] [N] [R] [N]".to_string(),
            "[M] [R] [R] [P] [M] [T] [H] [Q] [C]".to_string(),
            "[F] [F] [Z] [H] [S] [Z] [T] [D] [S]".to_string(),
            "[P] [H] [P] [Q] [P] [M] [P] [F] [D]".to_string(),
            " 1   2   3   4   5   6   7   8   9 ".to_string()
        ];
        
        assert_eq!(StackCollection { _storage: expected_stacks }, StackCollection::new(&stack_lines));
    }
}