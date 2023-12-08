#[derive(PartialEq, Debug)]
pub struct TreeGrid {
    storage: Vec<Vec<Tree>>,
}

impl TreeGrid {
    pub fn parse<T: Iterator<Item = String>>(iter: T) -> Result<TreeGrid, String> {
        let mut storage = Vec::new();
        let mut line_length: Option<usize> = None;

        for (row_index, line) in iter.enumerate() {
            if let Some(len) = line_length {
                if line.len() != len {
                    return Err(format!(
                        "Line length mismatch at row {}. Expected length {}, found length {}",
                        row_index + 1,
                        len,
                        line.len()
                    ));
                }
            } else {
                line_length = Some(line.len());
            }

            let mut tree_line = Vec::new();

            for (char_index, char) in line.chars().enumerate() {
                match char.to_digit(10) {
                    Some(height) => tree_line.push(Tree {
                        height,
                        row: row_index + 1,
                        column: char_index + 1,
                    }),
                    None => {
                        return Err(format!(
                            "Invalid character '{}' at row {}, position {}",
                            char,
                            row_index + 1,
                            char_index + 1
                        ))
                    }
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
        self.left_visibility(tree).visible
            || self.right_visibility(tree).visible
            || self.top_visibility(tree).visible
            || self.bottom_visibility(tree).visible
    }

    pub fn scenic_score(&self, tree: &Tree) -> usize {
        self.left_visibility(tree).count
            * self.right_visibility(tree).count
            * self.top_visibility(tree).count
            * self.bottom_visibility(tree).count
    }

    fn left_visibility(&self, tree: &Tree) -> TreeVisibility {
        self.check_visibility(tree, (0..tree.column - 1).rev(), |i| {
            &self.storage[tree.row - 1][i]
        })
    }

    fn right_visibility(&self, tree: &Tree) -> TreeVisibility {
        self.check_visibility(tree, tree.column..self.width(), |i| {
            &self.storage[tree.row - 1][i]
        })
    }

    fn top_visibility(&self, tree: &Tree) -> TreeVisibility {
        self.check_visibility(tree, (0..tree.row - 1).rev(), |i| {
            &self.storage[i][tree.column - 1]
        })
    }

    fn bottom_visibility(&self, tree: &Tree) -> TreeVisibility {
        self.check_visibility(tree, tree.row..self.height(), |i| {
            &self.storage[i][tree.column - 1]
        })
    }

    fn check_visibility<'a, I, F>(&'a self, tree: &Tree, range: I, tree_selector: F) -> TreeVisibility
    where
        I: Iterator<Item = usize>,
        F: Fn(usize) -> &'a Tree,
    {
        let mut count = 0;
        for i in range {
            count += 1;
            let checked_tree = tree_selector(i);
            if checked_tree.height >= tree.height {
                return TreeVisibility { count, visible: false }
            }
        }
        TreeVisibility { count, visible: true }
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
    column: usize,
}

struct TreeVisibility {
    count: usize,
    visible: bool,
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
            Tree {
                height: 1,
                row: 1,
                column: 1,
            },
            Tree {
                height: 2,
                row: 1,
                column: 2,
            },
            Tree {
                height: 3,
                row: 1,
                column: 3,
            },
            Tree {
                height: 4,
                row: 2,
                column: 1,
            },
            Tree {
                height: 5,
                row: 2,
                column: 2,
            },
            Tree {
                height: 6,
                row: 2,
                column: 3,
            },
        ];

        assert_eq!(
            expected_trees.iter().collect::<Vec<_>>(),
            tree_iter.collect::<Vec<_>>()
        );
    }

    #[test]
    fn handles_invalid_chars() {
        let lines = vec![
            "123".to_string(),
            "45a".to_string()
        ];
        let result = TreeGrid::parse(lines.into_iter());

        assert_eq!(
            Err("Invalid character 'a' at row 2, position 3".to_string()),
            result
        );
    }

    #[test]
    fn handles_unequal_length() {
        let lines = vec![
            "123".to_string(),
            "4567".to_string()
        ];
        let result = TreeGrid::parse(lines.into_iter());
        assert_eq!(
            Err("Line length mismatch at row 2. Expected length 3, found length 4".to_string()),
            result
        );
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
        lines = vec!["131".to_string(), "321".to_string(), "131".to_string()];
        assert!(center_visible(lines));

        // Center tree is NOT visible if trees on all sides are equal
        lines = vec!["121".to_string(), "222".to_string(), "121".to_string()];
        assert!(!center_visible(lines));

        // Center tree is NOT visible if trees on all sides are taller
        lines = vec!["131".to_string(), "323".to_string(), "131".to_string()];
        assert!(!center_visible(lines));
    }

    #[test]
    fn calculates_scenic_score() {
        let lines = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string()];
        let grid = TreeGrid::parse(lines.into_iter()).unwrap();
        let trees: Vec<_> = grid.tree_iter().collect();

        assert_eq!(4, grid.scenic_score(trees[7]));
        assert_eq!(8, grid.scenic_score(trees[17]));
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
