mod tree_grid;

use std::{
    fs::File,
    io::{self, BufRead},
};

use self::tree_grid::TreeGrid;

pub fn run(path: &str) -> (usize, usize) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let lines_iter = reader.lines().map(|x| x.unwrap());

    let tree_grid = TreeGrid::parse(lines_iter).unwrap();
    let visible_from_outside_count = tree_grid
        .tree_iter()
        .filter(|x| tree_grid.tree_visible(x))
        .count();
    let highest_scenic_score = tree_grid
        .tree_iter()
        .map(|x| tree_grid.scenic_score(x))
        .max()
        .unwrap();

    (visible_from_outside_count, highest_scenic_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_answer() {
        let result = run("inputs/day8.txt");
        println!("{:?}", result);
    }
}
