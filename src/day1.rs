use crate::sonar::*;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Depth {
    Depth::from(input)
}

#[aoc(day1, part1)]
pub fn part1(depth: &Depth) -> usize {
    depth.get_increases()
}