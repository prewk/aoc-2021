use crate::bingo::*;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Bingo {
    Bingo::from(input)
}

#[aoc(day4, part1)]
pub fn part1(bingo: &Bingo) -> u64 {
    let mut mut_bingo = bingo.clone();

    mut_bingo.play().unwrap()
}

#[aoc(day4, part2)]
pub fn part2(bingo: &Bingo) -> u64 {
    let mut mut_bingo = bingo.clone();

    mut_bingo.play_last().unwrap()
}
