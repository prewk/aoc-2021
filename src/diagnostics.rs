use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Diagnostics {
    bits: usize,
    report: Vec<usize>,
}

impl From<&str> for Diagnostics {
    fn from(input: &str) -> Self {
        Diagnostics {
            bits: input.lines().nth(0).expect("Expected at least one line").len(),
            report: input
                .lines()
                .map(|line| usize::from_str_radix(line, 2).expect("Invalid binary line"))
                .collect()
        }
    }
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

            if count > half {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_consumption() {
        let diagnostics = Diagnostics::from("00100\n\
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
                                                        01010");

        assert_eq!(diagnostics.get_power_consumption(), 198)
    }
}