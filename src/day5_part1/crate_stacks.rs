use indexmap::IndexMap;
use regex::Regex;

use super::lib::Stack;
use super::commands::Commands;

#[derive(PartialEq, Debug)]
pub struct CrateStacks<'a> {
    storage: IndexMap<&'a str, Stack<Crate>>
}

impl<'a> CrateStacks<'a> {
    pub fn new(stack_lines: &'a Vec<String>) -> Result<CrateStacks<'a>, String> {
        let (id_line, content_lines) = stack_lines.split_last().unwrap();
        let id_regex = Regex::new(r"\S+").unwrap();

        let mut id_lookup = IndexMap::new();
        for r#match in id_regex.find_iter(id_line) {
            let id = r#match.as_str();
            match id_lookup.entry(id) {
                indexmap::map::Entry::Vacant(_) => {
                    id_lookup.insert(id, r#match.start());
                },
                indexmap::map::Entry::Occupied(_) => {
                    return Err(format!("duplicate stack id '{}'", id));
                },
            }
        }

        let mut result = CrateStacks {
            storage: id_lookup
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

                let stack = result.storage.get_mut(id).unwrap();
                stack.push(Crate(crate_char));
            }
        }

        Ok(result)
    }

    pub fn update(&mut self, commands: Commands) {
        for command in commands {
            for _ in 0..command.count {
                let from_stack = self.storage.get_mut(command.from).unwrap();
                let item = from_stack.pop().unwrap();

                let to_stack = self.storage.get_mut(command.to).unwrap();
                to_stack.push(item);
            }
        }
    }

    pub fn tops_string(&self) -> String {
        self.storage
            .iter()
            .filter(|(_, c)| c.top().is_some())
            .map(|(_, c)| c.top().unwrap().0)
            .collect()
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
    mod new {
        use crate::day5_part1::crate_stacks::*;

        #[test]
        fn initializes_crate_stacks() {
            let mut expected_stacks = IndexMap::<_, Stack<Crate>>::new();
            expected_stacks.insert("2", "PFMQWGRT".into());
            expected_stacks.insert("-1", "HFR".into());
            expected_stacks.insert("a", "PZRVGHSD".into());
            expected_stacks.insert(",", "QHPBFWG".into());
            expected_stacks.insert("multi", "PSMJH".into());
            expected_stacks.insert("1", "MZTHSRPL".into());

            let stack_lines = vec![
                "[T]     [D]            [L]".to_string(),
                "[R]     [S] [G]        [P]".to_string(),
                "[G]     [H] [W]        [R]".to_string(),
                "[W]     [G] [F] [H]    [S]".to_string(),
                "[Q]     [V] [B] [J]    [H]".to_string(),
                "[M] [R] [R] [P] [M]    [T]".to_string(),
                "[F] [F] [Z] [H] [S]    [Z]".to_string(),
                "[P] [H] [P] [Q] [P]    [M]".to_string(),
                " 2   -1  a   ,   multi  1 ".to_string()
            ];
            
            assert_eq!(Ok(CrateStacks { storage: expected_stacks }), CrateStacks::new(&stack_lines));
        }

        #[test]
        fn handles_duplicate_ids() {
            let mut expected_stacks = IndexMap::<_, Stack<Crate>>::new();
            expected_stacks.insert("1", "PFMQWGRT".into());
            expected_stacks.insert("1", "HFR".into());

            let stack_lines = vec![
                "[F] [F]".to_string(),
                "[P] [H]".to_string(),
                " 1   1 ".to_string()
            ];
            
            assert_eq!(Err("duplicate stack id '1'".to_string()), CrateStacks::new(&stack_lines));
        }
    }

    mod update {
        use crate::day5_part1::crate_stacks::*;

        #[test]
        fn updates_crate_stack_using_commands() {
            let stack_lines = vec![
                "[M]        ".to_string(),
                "[F]     [Z]".to_string(),
                "[P] [H] [P]".to_string(),
                " 1   2   3 ".to_string()
            ];
            let mut stacks = CrateStacks::new(&stack_lines).unwrap();

            let command_lines = vec![
                "move 3 from 1 to 2".to_string(),
                "move 2 from 3 to 1".to_string()
            ];
            let commands = Commands::new(&command_lines).unwrap();

            stacks.update(commands);

            let expected_stack_lines = vec![
                "    [P]    ".to_string(),
                "    [F]    ".to_string(),
                "[P] [M]    ".to_string(),
                "[Z] [H]    ".to_string(),
                " 1   2   3 ".to_string()
            ];
            assert_eq!(CrateStacks::new(&expected_stack_lines).unwrap(), stacks)
        }
    }

    mod tops_string {
        use crate::day5_part1::crate_stacks::*;

        #[test]
        fn returns_tops_of_the_stacks_as_string() {
            let stack_lines = vec![
                "[M]        ".to_string(),
                "[F]     [Z]".to_string(),
                "[P] [H] [P]".to_string(),
                " 1   2   3 ".to_string()
            ];
            
            assert_eq!("MHZ", CrateStacks::new(&stack_lines).unwrap().tops_string());
        }

        #[test]
        fn skips_empty_stacks_when_returning_tops() {
            let stack_lines = vec![
                "[M]        ".to_string(),
                "[F]     [Z]".to_string(),
                "[P]     [P]".to_string(),
                " 1   2   3 ".to_string()
            ];
            
            assert_eq!("MZ", CrateStacks::new(&stack_lines).unwrap().tops_string());
        }
    }
}