use crate::chiton::*;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Chitons { Chitons::from(input) }

#[aoc(day15, part1)]
pub fn part1(chitons: &Chitons) -> i64 {
    chitons.path_cost().unwrap()
}

#[aoc(day15, part2)]
pub fn part2(chitons: &Chitons) -> i64 {
    chitons.grow().path_cost().unwrap()
}