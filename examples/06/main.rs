use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

fn count(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    // TODO: try `.entry().or_insert_with()`
    if n < 9 {
        1
    } else if let Some(&result) = cache.get(&n) {
        result
    } else {
        let result = count(n - 7, cache) + count(n - 9, cache);
        cache.insert(n, result);
        result
    }
}

fn part_one(contents: &String, days: u64) -> u64 {
    let mut cache = HashMap::new();
    contents
        .lines()
        .flat_map(|line| line.split(',').map(|x| u64::from_str(x).unwrap_or(0)))
        .fold(0, |acc, d| acc + count(days + (8 - d), &mut cache))
}

fn part_two(contents: &String, days: u64) -> u64 {
    part_one(contents, days)
}

fn main() {
    let contents =
        read_to_string("examples/06/main.in").expect("Something went wrong reading the file");

    println!(
        "How many lanternfish would there be after 80 days?\n- {}",
        part_one(&contents, 80)
    );

    println!(
        "How many lanternfish would there be after 256 days?\n- {}",
        part_two(&contents, 256)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/06/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(26, part_one(&get_contents(), 18));
    }
}
