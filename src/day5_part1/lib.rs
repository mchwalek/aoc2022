use std::io;

#[derive(PartialEq, Debug, Clone)]
pub struct Stack<T> {
    storage: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { storage: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.storage.push(item);
    }

    pub fn top(&self) -> Option<&T> {
        self.storage.last()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.storage.pop()
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut result = Stack::new();

        for item in iter {
            result.push(item);
        }

        result
    }
}

pub fn split_lines<T: Iterator<Item  = io::Result<String>>>(mut iter: T) -> (Vec<String>, Vec<String>) {
    let mut stack_lines = Vec::new();
    loop {
        match iter.next() {
            Some(Ok(line)) if !line.is_empty() => stack_lines.push(line),
            _ => break,
        }
    }

    let mut command_lines = Vec::new();
    loop {
        match iter.next() {
            Some(Ok(line)) => command_lines.push(line),
            _ => break,
        }
    }

    (stack_lines, command_lines)
}

#[cfg(test)]
mod tests {
    mod stack {
        use crate::day5_part1::lib::*;

        #[test]
        fn adds_items_to_top() {
            let mut stack = Stack::new();
            stack.push(1);
            stack.push(2);

            assert_eq!(Some(&2), stack.top());
        }

        #[test]
        fn removes_item_from_top_if_any() {
            let mut stack = Stack::new();
            stack.push(1);

            assert_eq!(Some(1), stack.pop());
            assert_eq!(None, stack.pop());
        }
    }

    mod split_lines {
        use crate::day5_part1::lib::*;

        #[test]
        fn splits_file_into_stack_and_command_lines() {
            let file_lines = vec![
                Ok("stack1".to_string()),
                Ok("stack2".to_string()),
                Ok("".to_string()),
                Ok("command1".to_string()),
                Ok("command2".to_string()),
            ];

            let (stack_lines, command_lines) = split_lines(file_lines.into_iter());
            let expected_stack_lines = vec!["stack1".to_string(), "stack2".to_string()];
            let expected_command_lines = vec!["command1".to_string(), "command2".to_string()];

            assert_eq!(expected_stack_lines, stack_lines);
            assert_eq!(expected_command_lines, command_lines);
        }
    }
}