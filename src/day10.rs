use crate::bracket::*;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<Bracket>> {
    parse_lines(input)
}

#[aoc(day10, part1)]
pub fn part1(lines: &[Vec<Bracket>]) -> usize {
    calc_syntax_score(lines)
}

#[aoc(day10, part2)]
pub fn part2(lines: &[Vec<Bracket>]) -> usize {
    middle_complete_score(lines)
}
