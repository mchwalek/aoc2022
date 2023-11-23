#[derive(PartialEq, Debug)]
pub struct TreeGrid {
    storage: Vec<Vec<Tree>>
}

impl TreeGrid {
    pub fn parse<T: Iterator<Item = String>>(iter: T) -> Result<TreeGrid, String> {
        let mut storage = Vec::new();
        let mut line_length: Option<usize> = None;

        for (row_index, line) in iter.enumerate() {
            if let Some(len) = line_length {
                if line.len() != len {
                    return Err(format!("Line length mismatch at row {}. Expected length {}, found length {}", row_index + 1, len, line.len()));
                }
            } else {
                line_length = Some(line.len());
            }

            let mut tree_line = Vec::new();

            for (char_index, char) in line.chars().enumerate() {
                match char.to_digit(10) {
                    Some(height) => tree_line.push(Tree { height, row: row_index + 1, column: char_index + 1 }),
                    None => return Err(format!("Invalid character '{}' at row {}, position {}", char, row_index + 1, char_index + 1)),
                }
            }

            storage.push(tree_line);
        }

        Ok(TreeGrid { storage })
    }

    pub fn tree_iter(&self) -> impl Iterator<Item = &Tree> {
        self.storage.iter().flatten()
    }

    pub fn tree_visible(&self, tree: &Tree) -> bool {
        let height = self.storage.len();
        let width = self.storage.first().map_or(0, |x| x.len());

        if self.edge_tree(tree) {
            return true;
        }

        self.visible_from_left(tree) ||
            self.visible_from_right(tree) ||
            self.visible_from_top(tree) ||
            self.visible_from_bottom(tree)
    }

    fn edge_tree(&self, tree: &Tree) -> bool {
        tree.row == 1 || tree.column == 1 || tree.row == self.height() || tree.column == self.width()
    }

    fn visible_from_left(&self, tree: &Tree) -> bool {
        for i in 0..tree.column - 1 {
            let checked_tree = &self.storage[tree.row - 1][i];
            if checked_tree.height >= tree.height {
                return false
            }
        }

        true
    }

    fn visible_from_right(&self, tree: &Tree) -> bool {
        for i in tree.column..self.width() {
            let checked_tree = &self.storage[tree.row - 1][i];
            if checked_tree.height >= tree.height {
                return false
            }
        }

        true
    }

    fn visible_from_top(&self, tree: &Tree) -> bool {
        for i in 0..tree.row - 1 {
            let checked_tree = &self.storage[i][tree.column - 1];
            if checked_tree.height >= tree.height {
                return false
            }
        }

        true
    }

    fn visible_from_bottom(&self, tree: &Tree) -> bool {
        for i in tree.row..self.height() {
            let checked_tree = &self.storage[i][tree.column - 1];
            if checked_tree.height >= tree.height {
                return false
            }
        }

        true
    }


    fn height(&self) -> usize {
        self.storage.len()
    }

    fn width(&self) -> usize {
        self.storage.first().map_or(0, |x| x.len())
    }
}

#[derive(PartialEq, Debug)]
pub struct Tree {
    height: u32,
    row: usize,
    column: usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterates_over_trees() {
        let lines = vec![
            "123".to_string(),
            "456".to_string(),
        ];
        let grid = TreeGrid::parse(lines.into_iter()).unwrap();

        let tree_iter = grid.tree_iter();
        let expected_trees = vec![
            Tree { height: 1, row: 1, column: 1 }, Tree { height: 2, row: 1, column: 2 }, Tree { height: 3, row: 1, column: 3 },
            Tree { height: 4, row: 2, column: 1 }, Tree { height: 5, row: 2, column: 2 }, Tree { height: 6, row: 2, column: 3 },
        ];

        assert_eq!(expected_trees.iter().collect::<Vec<_>>(), tree_iter.collect::<Vec<_>>());
    }

    #[test]
    fn handles_invalid_chars() {
        let lines = vec![
            "123".to_string(),
            "45a".to_string()
        ];
        let result = TreeGrid::parse(lines.into_iter());

        assert_eq!(Err("Invalid character 'a' at row 2, position 3".to_string()), result);
    }

    #[test]
    fn handles_unequal_length() {
        let lines = vec![
            "123".to_string(),
            "4567".to_string()
        ];
        let result = TreeGrid::parse(lines.into_iter());
        assert_eq!(Err("Line length mismatch at row 2. Expected length 3, found length 4".to_string()), result);
    }

    #[test]
    fn detects_if_tree_is_visible() {
        // Edge trees are always visible
        let mut lines = vec![
            "11".to_string(),
            "11".to_string(),
        ];
        assert_all_visible(lines);

        // Center tree is visible if any trees on one side are shorter
        lines = vec![
            "131".to_string(),
            "321".to_string(),
            "131".to_string(),
        ];
        assert!(center_visible(lines));

        // Center tree is NOT visible if trees on all sides are equal
        lines = vec![
            "121".to_string(),
            "222".to_string(),
            "121".to_string(),
        ];
        assert!(!center_visible(lines));

        // Center tree is NOT visible if trees on all sides are taller
        lines = vec![
            "131".to_string(),
            "323".to_string(),
            "131".to_string(),
        ];
        assert!(!center_visible(lines));
    }

    fn assert_all_visible(lines: Vec<String>) {
        let grid = TreeGrid::parse(lines.into_iter()).unwrap();
        let trees: Vec<_> = grid.tree_iter().collect();
        assert!(trees.into_iter().all(|x| grid.tree_visible(x)));
    }

    fn center_visible(lines: Vec<String>) -> bool {
        let grid = TreeGrid::parse(lines.into_iter()).unwrap();
        let trees: Vec<_> = grid.tree_iter().collect();
        grid.tree_visible(trees[4])
    }
}
