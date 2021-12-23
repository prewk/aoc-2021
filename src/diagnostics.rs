use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Diagnostics {
    bits: usize,
    report: Vec<usize>,
}

impl From<&str> for Diagnostics {
    fn from(input: &str) -> Self {
        Diagnostics {
            bits: input
                .lines()
                .next()
                .expect("Expected at least one line")
                .len(),
            report: input
                .lines()
                .map(|line| usize::from_str_radix(line, 2).expect("Invalid binary line"))
                .collect(),
        }
    }
}

pub enum RatingMethod {
    Oxygen,
    Co2,
}

impl Diagnostics {
    pub fn get_gamma_epsilon(&self) -> (usize, usize) {
        let mut ones: HashMap<usize, usize> = HashMap::new();
        let two = 2;
        let half = (self.report.len() / 2) as usize;

        for line in &self.report {
            for bit in (0..self.bits).rev() {
                let pos_val = u64::pow(two, bit as u32) as usize;

                *ones.entry(bit).or_insert(0) += if line & pos_val > 0 { 1 } else { 0 };
            }
        }

        let mut gamma = 0;
        let mut epsilon_mask = 0;
        for bit in (0..self.bits).rev() {
            let count = *ones.get(&bit).unwrap_or(&0);
            let pos_val = u64::pow(two, bit as u32) as usize;

            if count >= half {
                gamma += pos_val;
            }

            epsilon_mask += pos_val;
        }

        (gamma, gamma ^ epsilon_mask)
    }

    pub fn get_power_consumption(&self) -> usize {
        let (gamma, epsilon) = self.get_gamma_epsilon();

        gamma * epsilon
    }

    fn get_rating(&self, method: RatingMethod) -> Option<usize> {
        let two = 2;
        let mut keepers: Vec<usize> = self.report.clone();
        let mut rating = None;

        for bit in (0..self.bits).rev() {
            let pos_val = u64::pow(two, bit as u32) as usize;

            let ones = keepers.iter().filter(|value| *value & pos_val == pos_val).count();
            let zeroes = keepers.iter().filter(|value| *value & pos_val == 0).count();
            let half = (keepers.len() as f64 / 2.0).ceil() as usize;

            let next_keepers: Vec<usize> = keepers.iter().filter(|value| match method {
                RatingMethod::Oxygen => match ones >= half {
                    true => *value & pos_val == pos_val,
                    false => *value & pos_val == 0,
                },
                RatingMethod::Co2 => match zeroes <= ones {
                    true => *value & pos_val == 0,
                    false => *value & pos_val == pos_val,
                },
            }).copied().collect();

            if next_keepers.len() == 1 {
                rating = next_keepers.get(0).copied();
                break;
            } else if next_keepers.is_empty() {
                rating = keepers.last().copied();
                break;
            }

            keepers = next_keepers;
        }

        rating
    }

    pub fn get_life_support_rating(&self) -> Option<usize> {
        let oxygen = self.get_rating(RatingMethod::Oxygen)?;
        let co2 = self.get_rating(RatingMethod::Co2)?;

        Some(oxygen * co2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_consumption() {
        let diagnostics = Diagnostics::from(
            "00100\n\
                                                        11110\n\
                                                        10110\n\
                                                        10111\n\
                                                        10101\n\
                                                        01111\n\
                                                        00111\n\
                                                        11100\n\
                                                        10000\n\
                                                        11001\n\
                                                        00010\n\
                                                        01010",
        );

        assert_eq!(diagnostics.get_power_consumption(), 198)
    }

    #[test]
    fn test_life_support_rating() {
        let diagnostics = Diagnostics::from(
            "00100\n\
                                                        11110\n\
                                                        10110\n\
                                                        10111\n\
                                                        10101\n\
                                                        01111\n\
                                                        00111\n\
                                                        11100\n\
                                                        10000\n\
                                                        11001\n\
                                                        00010\n\
                                                        01010",
        );

        assert_eq!(diagnostics.get_life_support_rating(), Some(230));
    }
}
