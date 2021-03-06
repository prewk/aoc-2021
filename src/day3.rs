use crate::diagnostics::*;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Diagnostics {
    Diagnostics::from(input)
}

#[aoc(day3, part1)]
pub fn part1(diagnostics: &Diagnostics) -> usize {
    diagnostics.get_power_consumption()
}

#[aoc(day3, part2)]
pub fn part2(diagnostics: &Diagnostics) -> usize {
    diagnostics.get_life_support_rating().unwrap()
}
