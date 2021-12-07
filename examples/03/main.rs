use std::fs::read_to_string;

fn part_one(contents: &String) -> i32 {
    return contents
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
                    return if *ones > (size - ones) {
                        (gamma + 2_i32.pow(n_bits as u32), epsilon)
                    } else if *ones < (size - ones) {
                        (gamma, epsilon + 2_i32.pow(n_bits as u32))
                    } else {
                        (gamma, epsilon)
                    };
                });
            return t.0 * t.1;
        })
        .unwrap();
}

#[derive(Debug)]
struct Node {
    count: u32,
    // zero
    left: Option<Box<Node>>,
    // one
    right: Option<Box<Node>>,
}

impl Node {
    fn new() -> Node {
        Node {
            count: 0,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: &str) {
        value.chars().nth(0).map(|c| {
            self.count += 1;
            let target_node = if c == '0' {
                &mut self.left
            } else {
                &mut self.right
            };

            match target_node {
                Some(child) => child.insert(&value[1..]),
                None => {
                    let mut new_child = Node::new();
                    new_child.insert(&value[1..]);
                    *target_node = Some(Box::new(new_child));
                }
            }
        });
    }

    fn oxygen_generator_rating(&self) -> u32 {
        return self.calculate(0, |l, r| l > r);
    }

    fn co2_scrubber_rating(&self) -> u32 {
        return self.calculate(0, |l, r| l <= r);
    }

    fn calculate(&self, value: u32, pred: impl Fn(u32, u32) -> bool) -> u32 {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Some(left), Some(right)) => {
                if pred(left.count, right.count) {
                    left.calculate(value << 1, pred)
                } else {
                    right.calculate((value ^ 1) << 1, pred)
                }
            }
            (Some(left), None) => left.calculate(value << 1, pred),
            (None, Some(right)) => right.calculate((value ^ 1) << 1, pred),
            (_, _) => value >> 1,
        }
    }
}

fn part_two(contents: &String) -> u32 {
    let mut root = Node::new();

    // Populate the tree
    contents.lines().for_each(|l| root.insert(l));

    return root.oxygen_generator_rating() * root.co2_scrubber_rating();
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
        return read_to_string("examples/03/sample.in")
            .expect("Something went wrong reading the file");
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
