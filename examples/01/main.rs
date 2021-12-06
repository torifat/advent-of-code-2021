use itertools::Itertools;
use std::fs::read_to_string;
use std::str::FromStr;

fn part_one(contents: &String) -> u16 {
  return contents
    .lines()
    .flat_map(u16::from_str)
    .tuple_windows::<(u16, u16)>()
    .fold(0, |acc, (l, r)| if r > l { acc + 1 } else { acc });
}

fn part_two(contents: &String) -> u16 {
  return contents
    .lines()
    .flat_map(u16::from_str)
    .tuple_windows::<(u16, u16, u16)>()
    .map(|(a, b, c)| a + b + c)
    .tuple_windows::<(u16, u16)>()
    .fold(0, |acc, (l, r)| if r > l { acc + 1 } else { acc });
}

fn main() {
  let contents =
    read_to_string("examples/01/main.in").expect("Something went wrong reading the file");

  println!(
    "There are {} measurements that are larger than the previous measurement.",
    part_one(&contents)
  );

  println!(
    "There are {} sums that are larger than the previous sum.",
    part_two(&contents)
  );
}
#[cfg(test)]
mod tests {
  use super::*;

  fn get_contents() -> String {
    return read_to_string("examples/01/sample.in").expect("Something went wrong reading the file");
  }

  #[test]
  fn test_part_one() {
    assert_eq!(7, part_one(&get_contents()));
  }

  #[test]
  fn test_part_two() {
    assert_eq!(5, part_two(&get_contents()));
  }
}
