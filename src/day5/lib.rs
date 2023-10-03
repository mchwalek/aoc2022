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

    pub fn push_many<U: Iterator<Item = T>>(&mut self, iter: U) {
        for item in iter {
            self.storage.push(item);
        }
    }

    pub fn top(&self) -> Option<&T> {
        self.storage.last()
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        self.storage.pop().ok_or("empty stack")
    }

    pub fn pop_many_iter<'a>(&'a mut self, count: usize) -> Result<impl Iterator<Item = T> + 'a, String> {
        if self.len() < count {
            return Err(format!("not enough items ({}) in stack", self.len()));
        }

        let drain_start = self.storage.len() - count;
        Ok(self.storage.drain(drain_start..).rev())
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut result = Stack::new();
        result.push_many(iter.into_iter());

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
        use crate::day5::lib::*;

        #[test]
        fn adds_items_to_top() {
            let mut stack = Stack::new();
            stack.push(1);
            stack.push(2);

            assert_eq!(Some(&2), stack.top());
        }

        #[test]
        fn adds_many_items_to_top() {
            let items = vec![1, 2];

            let mut stack = Stack::new();
            stack.push_many(items.into_iter());

            assert_eq!(Some(&2), stack.top());
        }

        #[test]
        fn removes_item_from_top_if_possible() {
            let mut stack = Stack::new();
            stack.push(1);

            assert_eq!(Ok(1), stack.pop());
            assert_eq!(Err("empty stack"), stack.pop());
        }

        #[test]
        fn removes_many_items_from_top_if_possible() {
            let mut stack = Stack::new();
            stack.push(1);
            stack.push(2);
            let mut stack2 = stack.clone();

            assert_eq!(Ok(vec![2, 1]), stack.pop_many_iter(2).map(|x| x.collect()));
            assert_eq!(Err::<Vec<_>, _>("not enough items (2) in stack".to_string()), stack2.pop_many_iter(3).map(|x| x.collect()));
        }
    }

    mod split_lines {
        use crate::day5::lib::*;

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