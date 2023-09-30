#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

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