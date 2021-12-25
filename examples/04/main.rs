#![feature(control_flow_enum)]
mod bingo;

use bingo::Bingo;
use std::fs::read_to_string;

fn part_one(contents: &String) -> i32 {
    Bingo::init(contents).roll(false) as i32
}

fn part_two(contents: &String) -> i32 {
    Bingo::init(contents).roll(true) as i32
}

fn main() {
    let contents =
        read_to_string("examples/04/main.in").expect("Something went wrong reading the file");

    println!(
        "What will your final score be if you choose that board?\n- {}",
        part_one(&contents)
    );

    println!(
        "Once it wins, what would its final score be?\n- {}",
        part_two(&contents)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/04/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(4512, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1924, part_two(&get_contents()));
    }
}
