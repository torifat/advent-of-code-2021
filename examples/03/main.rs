mod node;

use node::Node;
use std::fs::read_to_string;

fn part_one(contents: &String) -> i32 {
    contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>()
        })
        .reduce(|acc, v| acc.iter().zip(v).map(|(x, y)| x + y).collect())
        .map(|v| {
            let size = contents.lines().count() as u32;
            let t = v
                .iter()
                .rev()
                .enumerate()
                .fold((0, 0), |(gamma, epsilon), (n_bits, ones)| {
                    if *ones > (size - ones) {
                        (gamma + 2_i32.pow(n_bits as u32), epsilon)
                    } else if *ones < (size - ones) {
                        (gamma, epsilon + 2_i32.pow(n_bits as u32))
                    } else {
                        (gamma, epsilon)
                    }
                });
            t.0 * t.1
        })
        .unwrap()
}

fn part_two(contents: &String) -> u32 {
    let mut root = Node::leaf();

    // Populate the tree
    contents.lines().for_each(|l| root.insert(l));

    root.oxygen_generator_rating() * root.co2_scrubber_rating()
}

fn main() {
    let contents =
        read_to_string("examples/03/main.in").expect("Something went wrong reading the file");

    println!(
        "What is the power consumption of the submarine?\n- {}",
        part_one(&contents)
    );

    println!(
        "What is the life support rating of the submarine?\n- {}",
        part_two(&contents)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/03/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(198, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(230, part_two(&get_contents()));
    }
}
