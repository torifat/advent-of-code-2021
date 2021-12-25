#[derive(Debug)]
pub struct Node {
    count: u32,
    // zero
    left: Option<Box<Node>>,
    // one
    right: Option<Box<Node>>,
}

impl Node {
    pub fn leaf() -> Box<Node> {
        Box::new(Node {
            count: 0,
            left: None,
            right: None,
        })
    }

    pub fn insert(&mut self, value: &str) {
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
                    let mut new_child = Node::leaf();
                    new_child.insert(&value[1..]);
                    *target_node = Some(new_child);
                }
            }
        });
    }

    pub fn oxygen_generator_rating(&self) -> u32 {
        self.calculate(0, |l, r| l > r)
    }

    pub fn co2_scrubber_rating(&self) -> u32 {
        self.calculate(0, |l, r| l <= r)
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
