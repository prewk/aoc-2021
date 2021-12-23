use crate::fish::*;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> FishyWaters {
    FishyWaters::from(input)
}

#[aoc(day6, part1)]
pub fn part1(waters: &FishyWaters) -> u64 {
    count_fishes(waters, 80)
}

#[aoc(day6, part2)]
pub fn part2(waters: &FishyWaters) -> u64 {
    count_fishes(waters, 256)
}