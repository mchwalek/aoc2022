use std::{
    fs::File,
    io::{self, BufRead},
};

fn run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut max_sum = 0;
    let mut sum = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        match line.parse::<i32>() {
            Ok(calories) => sum += calories,
            Err(_) => {
                if sum > max_sum {
                    max_sum = sum;
                }

                sum = 0;
            }
        }
    }

    max_sum
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
