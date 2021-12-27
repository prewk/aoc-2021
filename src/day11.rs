use crate::octopus::*;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Map { Map::from(input) }

#[aoc(day11, part1)]
pub fn part1(map: &Map) -> usize {
    let mut step = map.clone();

    for _ in 0..100 {
        step = step.tick();
    }

    step.flash_count
}

#[aoc(day11, part2)]
pub fn part2(map: &Map) -> usize {
    find_sync_flash_step(&map)
}