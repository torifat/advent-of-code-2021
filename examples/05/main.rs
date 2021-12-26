use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Point {
    fn from((x, y): (u16, u16)) -> Self {
        Point { x, y }
    }
}

type Line = (Point, Point);

fn is_vertical_line((p1, p2): &Line) -> bool {
    p1.x == p2.x
}

fn is_horizontal_line((p1, p2): &Line) -> bool {
    p1.y == p2.y
}

fn create_line((p1, p2): ((u16, u16), (u16, u16))) -> Line {
    (Point::from(p1.min(p2)), Point::from(p1.max(p2)))
}

fn count_clouds(contents: &String, include_diagonal: bool) -> usize {
    let lines: Vec<Line> = contents
        .lines()
        .flat_map(|line| {
            line.split(&[' ', ','][..])
                .filter(|s| *s != "->")
                .flat_map(u16::from_str)
                .tuples::<(_, _)>()
                .collect_tuple()
                .map(create_line)
        })
        .collect();

    let mut map = HashMap::new();

    lines
        .iter()
        .flat_map(|line| {
            let (p1, p2) = line;
            if is_vertical_line(&line) {
                (p1.y..=p2.y).map(|y| (p1.x, y)).collect()
            } else if is_horizontal_line(&line) {
                (p1.x..=p2.x).map(|x| (x, p1.y)).collect()
            } else {
                if include_diagonal {
                    if p1.y < p2.y {
                        (p1.x..=p2.x).zip(p1.y..=p2.y).collect()
                    } else {
                        (p1.x..=p2.x).zip((p2.y..=p1.y).rev()).collect()
                    }
                } else {
                    vec![]
                }
            }
        })
        .for_each(|(x, y)| *map.entry((x, y)).or_insert(0) += 1);

    map.iter().filter(|(_, v)| **v > 1).count()
}

fn part_one(contents: &String) -> usize {
    count_clouds(contents, false)
}

fn part_two(contents: &String) -> usize {
    count_clouds(contents, true)
}

fn main() {
    let contents =
        read_to_string("examples/05/main.in").expect("Something went wrong reading the file");

    println!(
        "At how many points do at least two lines overlap?\n- {}",
        part_one(&contents)
    );

    println!(
        "At how many points do at least two lines overlap?\n- {}",
        part_two(&contents)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_contents() -> String {
        read_to_string("examples/05/sample.in").expect("Something went wrong reading the file")
    }

    #[test]
    fn test_part_one() {
        assert_eq!(5, part_one(&get_contents()));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(12, part_two(&get_contents()));
    }
}
