use crate::submarine::*;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Submarine {
    Submarine::from(input)
}

#[aoc(day2, part1)]
pub fn part1(sub: &Submarine) -> u64 {
    let mut water = Water::submerge(sub);
    water.run1();

    water.position_mult()
}

#[aoc(day2, part2)]
pub fn part2(sub: &Submarine) -> u64 {
    let mut water = Water::submerge(sub);
    water.run2();

    water.position_mult()
}
