use std::collections::HashMap;

use regex::Regex;

use super::lib::Stack;

#[derive(PartialEq, Debug)]
pub struct CrateStacks<'a> {
    _storage: HashMap<&'a str, Stack<Crate>>
}

impl<'a> CrateStacks<'a> {
    pub fn new(stack_lines: &'a Vec<String>) -> CrateStacks<'a> {

        let (id_line, content_lines) = stack_lines.split_last().unwrap();

        let id_regex = Regex::new(r"\S+").unwrap();
        let id_lookup: HashMap<_, _> = id_regex
            .find_iter(id_line)
            .map(|m| (m.as_str(), m.start()))
            .collect();

        let mut result = CrateStacks {
            _storage: id_lookup
                .iter()
                .map(|(&id, _)| (id, Stack::new()))
                .collect()
        };

        for line in content_lines.into_iter().rev() {
            for (&id, &index) in id_lookup.iter() {
                let crate_char =  line.chars().nth(index).unwrap();
                if crate_char == ' ' {
                    continue;
                }

                let stack = result._storage.get_mut(id).unwrap();
                stack.push(Crate(crate_char));
            }
        }

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct Crate(pub char);

impl From<&str> for Stack<Crate> {
    fn from(value: &str) -> Self {
        value.chars().map(|x| Crate(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_crate_stacks() {
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
        expected_stacks.insert("10", "W".into());

        let stack_lines = vec![
            "[T]     [D]         [L]                 ".to_string(),
            "[R]     [S] [G]     [P]         [H]     ".to_string(),
            "[G]     [H] [W]     [R] [L]     [P]     ".to_string(),
            "[W]     [G] [F] [H] [S] [M]     [L]     ".to_string(),
            "[Q]     [V] [B] [J] [H] [N] [R] [N]    ".to_string(),
            "[M] [R] [R] [P] [M] [T] [H] [Q] [C]    ".to_string(),
            "[F] [F] [Z] [H] [S] [Z] [T] [D] [S]    ".to_string(),
            "[P] [H] [P] [Q] [P] [M] [P] [F] [D] [W]".to_string(),
            " 1   2   3   4   5   6   7   8   9   10".to_string()
        ];
        
        assert_eq!(CrateStacks { _storage: expected_stacks }, CrateStacks::new(&stack_lines));
    }
}