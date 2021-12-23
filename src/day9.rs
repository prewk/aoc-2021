use crate::smoke::*;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Map {
    Map::from(input)
}

#[aoc(day9, part1)]
pub fn part1(map: &Map) -> u32 {
    risk_sum(map)
}

#[aoc(day9, part2)]
pub fn part2(map: &Map) -> usize {
    map.mult_three_largest_basins()
}
