pub struct Crabs {
    positions: Vec<u64>,
    max: u64,
}

impl From<&str> for Crabs {
    fn from(input: &str) -> Self {
        let positions: Vec<u64> = input
            .split(',')
            .map(|v| v.parse::<u64>())
            .filter_map(Result::ok)
            .collect();
        let max: u64 = positions.iter().fold(0, |max, v| *match v > &max {
            true => v,
            false => &max,
        });

        Crabs { positions, max }
    }
}

pub enum FuelCalcMethod {
    Const,
    Var,
}

impl Crabs {
    fn calc_distances_for(&self, target: u64) -> Vec<u64> {
        self.positions
            .iter()
            .map(|pos| (*pos as i64 - target as i64).abs() as u64)
            .collect()
    }

    fn calc_const_fuel_for(&self, target: u64) -> u64 {
        self.calc_distances_for(target)
            .iter()
            .sum()
    }

    fn calc_var_fuel_for(&self, target: u64) -> u64 {
        self.calc_distances_for(target)
            .iter()
            .fold(0, |fuel, distance| {
                fuel + (0..*distance).fold(0, |i, d| i + (d + 1))
            })
    }

    pub fn test_all_const(&self, method: FuelCalcMethod) -> u64 {
        let mut cheapest = u64::MAX;

        for i in 1..=self.max {
            let fuel = match method {
                FuelCalcMethod::Const => self.calc_const_fuel_for(i),
                FuelCalcMethod::Var => self.calc_var_fuel_for(i),
            };

            if fuel < cheapest {
                cheapest = fuel;
            }
        }

        cheapest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let crabs = Crabs::from("16,1,2,0,4,2,7,1,2,14");

        assert_eq!(crabs.test_all_const(FuelCalcMethod::Const), 37);
        assert_eq!(crabs.test_all_const(FuelCalcMethod::Var), 168);
    }
}
