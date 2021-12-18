use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::ControlFlow;
use std::str::FromStr;

const SIZE: usize = 5;
const FIVE_SET_BITS: u16 = 2_u16.pow(SIZE as u32) - 1;
const ITEMS: usize = SIZE * SIZE;
type Rows = [u16; SIZE];

#[derive(Debug)]
pub struct Bingo {
    nums: Vec<u16>,
    bits: Vec<Rows>,
    boards: Vec<u16>,
    mapping: HashMap<u16, Vec<(u16, u16, u16)>>,
}

impl Bingo {
    pub fn init(contents: &String) -> Bingo {
        let mut lines = contents.lines();

        let nums = lines
            .next()
            .into_iter()
            .flat_map(|l| l.split(',').flat_map(u16::from_str))
            .collect();

        let (bits, boards, mapping) = lines.chunks(SIZE + 1).into_iter().enumerate().fold(
            (vec![], vec![], HashMap::new()),
            |(mut bits, boards, mapping), (b_id, chunk)| {
                bits.push([0, 0, 0, 0, 0]);
                let (new_mapping, new_boards) = chunk.skip(1).enumerate().fold(
                    (mapping, boards),
                    |(mut mapping, mut boards), (r_id, row)| {
                        row.split_whitespace()
                            .flat_map(u16::from_str)
                            .enumerate()
                            .for_each(|(c_id, num)| {
                                boards.push(num);
                                let vec = mapping.entry(num).or_insert(vec![]);
                                vec.push((b_id as u16, r_id as u16, c_id as u16));
                            });

                        return (mapping, boards);
                    },
                );
                return (bits, new_boards, new_mapping);
            },
        );

        return Bingo {
            nums,
            bits,
            boards,
            mapping,
        };
    }

    fn sum_of_unmarked_number(&self, b_id: u16) -> u16 {
        let from = (b_id as usize) * ITEMS;
        let to = from + ITEMS;
        return self.boards[from..to]
            .iter()
            .filter(|n| {
                let t =
                    self.mapping
                        .get(n)
                        .unwrap_or(&vec![])
                        .iter()
                        .any(|(i_b_id, r_id, c_id)| {
                            let row = self.bits[b_id as usize][*r_id as usize];
                            return b_id == *i_b_id && (row & (1 << c_id) == 0);
                        });
                return t;
            })
            .sum();
    }

    pub fn roll(&mut self, check_last: bool) -> u16 {
        let mut completed: HashSet<u16> = HashSet::with_capacity(self.bits.len());
        let mut last_completed: Option<(u16, u16)> = None;
        for n in self.nums.iter() {
            let out =
                self.mapping
                    .entry(*n)
                    .or_default()
                    .iter()
                    .try_for_each(|(b_id, r_id, c_id)| {
                        if !completed.contains(b_id) {
                            let rows = &mut (self.bits[(*b_id) as usize]);
                            let byte = &mut rows[*r_id as usize];
                            *byte |= 1 << *c_id;

                            if *byte == FIVE_SET_BITS
                                || rows.iter().fold(FIVE_SET_BITS, |acc, n| acc & *n) != 0
                            {
                                completed.insert(*b_id);
                                last_completed = Some((*b_id, *n));
                                if !check_last || completed.len() == completed.capacity() {
                                    return ControlFlow::Break((*b_id, *n));
                                }
                            }
                        }

                        ControlFlow::Continue(())
                    });
            if out.is_break() {
                break;
            }
        }

        return match last_completed {
            Some((b_id, n)) => {
                let sum = self.sum_of_unmarked_number(b_id);
                return sum * n;
            }
            None => 0,
        };
    }
}
