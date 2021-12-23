#[derive(Clone)]
pub struct FishyWaters {
    pub days: [u64; 9],
}

impl From<&str> for FishyWaters {
    fn from(input: &str) -> Self {
        let mut days: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u64>())
            .filter_map(Result::ok)
            .for_each(|day| {
                days[day as usize] += 1;
            });

        FishyWaters {
            days
        }
    }
}

pub fn count_fishes(waters: &FishyWaters, day_count: usize) -> u64 {
    let mut state = waters.days;

    for _ in 0..day_count {
        let mut wip = state;

        wip[8] = state[0];
        wip[7] = state[8];
        wip[6] = state[7] + state[0];
        wip[5] = state[6];
        wip[4] = state[5];
        wip[3] = state[4];
        wip[2] = state[3];
        wip[1] = state[2];
        wip[0] = state[1];

        state = wip
    }

    state.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fish_count() {
        let waters = FishyWaters::from("3,4,3,1,2");

        assert_eq!(count_fishes(&waters, 18), 26);
        assert_eq!(count_fishes(&waters, 80), 5934);
    }
}