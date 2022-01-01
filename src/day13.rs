use crate::fold::*;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Instructions {
    Instructions::from(input)
}

#[aoc(day13, part1)]
pub fn part1(instr: &Instructions) -> usize {
    Paper::from(instr).fold(instr.folds.first().unwrap()).count_visible_dots()
}