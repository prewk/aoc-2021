use crate::path::*;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Map { Map::from(input) }

#[aoc(day12, part1)]
pub fn part1(map: &Map) -> usize {
    map.paths().unwrap().len()
}