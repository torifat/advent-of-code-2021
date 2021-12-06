use itertools::Itertools;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
  let count = read_to_string("examples/01/main.in")
    .expect("Something went wrong reading the file")
    .lines()
    .flat_map(u16::from_str)
    .tuple_windows::<(u16, u16)>()
    .fold(0, |a, (l, r)| if r > l { a + 1 } else { a });

  println!(
    "There are {:?} measurements that are larger than the previous measurement.",
    count
  );
}
