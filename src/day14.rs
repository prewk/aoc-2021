use crate::poly::*;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Poly { Poly::from(input) }

#[aoc(day14, part1)]
pub fn part1(poly: &Poly) -> u64 {
    let mut m_poly = poly.clone();

    for _ in 0..10 {
        m_poly.next().unwrap();
    }

    m_poly.puzzle_output().unwrap()
}

#[aoc(day14, part2)]
pub fn part2(poly: &Poly) -> u64 {
    let mut m_poly = poly.clone();

    for _ in 0..40 {
        m_poly.next().unwrap();
    }

    m_poly.puzzle_output().unwrap()
}