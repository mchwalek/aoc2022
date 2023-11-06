use std::{
    fs::File,
    io::{self, BufRead},
};

fn run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut sums: Vec<i32> = Vec::new();
    let mut sum = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        match line.parse::<i32>() {
            Ok(calories) => sum += calories,
            Err(_) => {
                sums.push(sum);
                sum = 0;
            }
        }
    }

    sums.push(sum);

    sums.sort_by(|a, b| b.cmp(a));
    sums[0..3].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_answer() {
        let result = run("inputs/day1.txt");
        println!("{}", result);
    }
}
