use crate::segment::*;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Entries {
    Entries::from(input)
}

#[aoc(day8, part1)]
pub fn part1(entries: &Entries) -> usize {
    entries.count_easily_guessed_outputs()
}

#[aoc(day8, part2)]
pub fn part2(entries: &Entries) -> u64 {
    entries.count_real_output().unwrap()
}
