mod day1_part1;
mod day1_part2;
mod day2_part1;
mod day2_part2;
mod day3_part1;
mod day3_part2;
mod day4_part1;
mod day4_part2;
mod day5;
mod day6;
mod day7;

fn main() {
    day1_part1::run("inputs/day1.txt");
    day1_part2::run("inputs/day1.txt");
    day2_part1::run("inputs/day2.txt");
    day2_part2::run("inputs/day2.txt");
    day3_part1::run("inputs/day3.txt");
    day3_part2::run("inputs/day3.txt");
    day4_part1::run("inputs/day4.txt");
    day4_part2::run("inputs/day4.txt");
    day5::run_part1("inputs/day5.txt");
    day5::run_part2("inputs/day5.txt");
    day6::run("inputs/day6.txt");
    day7::run("inputs/day7.txt");
}
