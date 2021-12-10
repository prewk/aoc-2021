use crate::vents::*;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Map {
    Map::from(input)
}

#[aoc(day5, part1)]
pub fn part1(map: &Map) -> usize {
    map.get_count_without_diagonals()
}

#[aoc(day5, part2)]
pub fn part2(map: &Map) -> usize {
    map.get_count_with_diagonals()
}
