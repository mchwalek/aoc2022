mod tree_grid;

use std::{fs::File, io::{self, BufRead}};

use self::tree_grid::TreeGrid;

pub fn run(path: &str) -> usize {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let lines_iter = reader.lines().map(|x| x.unwrap());

    let tree_grid = TreeGrid::parse(lines_iter).unwrap();
    tree_grid.tree_iter().filter(|x| tree_grid.tree_visible(x)).count()
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
