use itertools::Itertools;
use std::fs::read_to_string;
use std::str::FromStr;

fn part_one(contents: &String) -> i32 {
    // TODO use a better solution - https://en.wikipedia.org/wiki/Geometric_median
    let items: Vec<i32> = contents
        .trim()
        .split(',')
        .flat_map(|x| i32::from_str(x))
        .sorted()
        .collect();

    // Sample and main have even numbers of input, however both n/2 & (n/2)+1 are the same
    let median = items[items.len() / 2];
    items.iter().map(|x| (x - median).abs()).sum()
}

fn part_two(contents: &String) -> i32 {
    let items: Vec<i32> = contents
        .trim()
        .split(',')
        .flat_map(|x| i32::from_str(x))
        .collect();

    let min = *items.iter().min().unwrap();
    let max = *items.iter().max().unwrap();

    (min..=max)
        .map(|x| {
            items
                .iter()
                .map(|y| {
                    let distance = (x - y).abs();
                    distance * (distance + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    let contents =
        read_to_string("examples/07/main.in").expect("Something went wrong reading the file");

    println!(
        "How much fuel must they spend to align to that position?\n- {}",
        part_one(&contents)
    );

    println!("?\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/07/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(37, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(168, part_two(&get_contents()));
    }
}
