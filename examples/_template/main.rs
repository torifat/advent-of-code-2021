use std::fs::read_to_string;

fn part_one(_contents: &String) -> u32 {
    1
}

fn part_two(_contents: &String) -> u32 {
    2
}

fn main() {
    let contents =
        read_to_string("examples/0X/main.in").expect("Something went wrong reading the file");

    println!("?\n- {}", part_one(&contents));

    println!("?\n- {}", part_two(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/0X/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2, part_two(&get_contents()));
    }
}
