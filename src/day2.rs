use crate::submarine::*;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Submarine { Submarine::from(input) }

#[aoc(day2, part1)]
pub fn part1(sub: &Submarine) -> u64 {
    let mut water = Water::submerge(sub);
    water.vroom();

    water.position_mult()
}