#[derive(PartialEq)]
pub struct Depth {
    depths: Vec<u64>,
}

impl From<&str> for Depth {
    fn from(input: &str) -> Self {
        Depth {
            depths: input
                .lines()
                .map(|line| line.parse::<u64>())
                .filter_map(Result::ok)
                .collect()
        }
    }
}

impl Depth {
    pub fn get_increases(&self) -> usize {
        let mut increases = 0usize;
        let mut opt_last = None;
        for depth in &self.depths {
            if let Some(l) = opt_last {
                if depth > l {
                    increases += 1;
                }
            }

            opt_last = Some(depth);
        }

        increases
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increases() {
        let input = "199\n\
                           200\n\
                           208\n\
                           210\n\
                           200\n\
                           207\n\
                           240\n\
                           269\n\
                           260\n\
                           263";
        let depth = Depth::from(input);

        assert_eq!(depth.get_increases(), 7);
    }
}