use itertools::Itertools;
use std::fs::read_to_string;
use std::str::FromStr;

fn part_one(contents: &String) -> i32 {
    let t = contents
        .lines()
        .flat_map(|s| s.split_whitespace().tuples())
        .map(|(c, v)| (c, i32::from_str(v).unwrap_or(0)))
        .fold((0, 0), |(h, d), command| match command {
            ("forward", x) => (h + x, d),
            ("up", x) => (h, d - x),
            ("down", x) => (h, d + x),
            (_, _) => (h, d),
        });

    t.0 * t.1
}

fn part_two(contents: &String) -> i32 {
    let t = contents
        .lines()
        .flat_map(|s| s.split_whitespace().tuples())
        .map(|(c, v)| (c, i32::from_str(v).unwrap_or(0)))
        .fold((0, 0, 0), |(h, d, a), command| match command {
            ("forward", x) => (h + x, d + (x * a), a),
            ("up", x) => (h, d, a - x),
            ("down", x) => (h, d, a + x),
            (_, _) => (h, d, a),
        });

    t.0 * t.1
}

fn main() {
    let contents =
        read_to_string("examples/02/main.in").expect("Something went wrong reading the file");

    println!(
        "What do you get if you multiply your final horizontal position by your final depth?\n- {}",
        part_one(&contents)
    );

    println!(
        "What do you get if you multiply your final horizontal position by your final depth?\n- {}",
        part_two(&contents)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/02/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(150, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(900, part_two(&get_contents()));
    }
}
