#[derive(Clone)]
pub struct FishyWaters {
    pub state: Vec<u64>,
}

impl From<&str> for FishyWaters {
    fn from(input: &str) -> Self {
        FishyWaters {
            state: input
                .lines()
                .nth(0)
                .unwrap()
                .split(',')
                .map(|n| n.parse::<u64>())
                .filter_map(Result::ok)
                .collect(),
        }
    }
}

impl FishyWaters {
    pub fn cycle(&self) -> FishyWaters {
        let mut newbies: Vec<u64> = vec![];
        let mut next_day: Vec<u64> = vec![];

        for prev in &self.state {
            let next = match prev {
                0 => {
                    newbies.push(8);
                    6
                }
                _ => prev - 1,
            };

            next_day.push(next);
        }

        next_day.append(&mut newbies);

        FishyWaters {
            state: next_day,
        }
    }

    pub fn len(&self) -> usize {
        self.state.len()
    }
}

pub fn get_fish_count(initial: &FishyWaters, days: u64) -> usize {
    let mut current = initial.clone();

    for i in 0..days {
        current = current.cycle();
    }

    current.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fish_count() {
        let waters = FishyWaters::from("3,4,3,1,2");

        assert_eq!(get_fish_count(&waters, 18), 26);
        assert_eq!(get_fish_count(&waters, 80), 5934);
    }
}