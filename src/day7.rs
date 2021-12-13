use crate::crabs::*;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Crabs {
    Crabs::from(input)
}

#[aoc(day7, part1)]
pub fn part1(crabs: &Crabs) -> u64 {
    crabs.test_all_const(FuelCalcMethod::Const)
}

#[aoc(day7, part2)]
pub fn part2(crabs: &Crabs) -> u64 {
    crabs.test_all_const(FuelCalcMethod::Var)
}
