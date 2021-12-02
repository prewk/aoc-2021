use crate::sonar::*;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Depth {
    Depth::from(input)
}

#[aoc(day1, part1)]
pub fn part1(depth: &Depth) -> u64 {
    depth.get_increases()
}

#[aoc(day1, part2)]
pub fn part2(depth: &Depth) -> u64 {
    depth.get_windowed_increases()
}