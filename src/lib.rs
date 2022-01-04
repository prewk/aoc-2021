extern crate aoc_runner;
extern crate either;
extern crate regex;
extern crate png_encode_mini;

#[macro_use]
extern crate aoc_runner_derive;

pub mod bingo;
pub mod bracket;
pub mod crabs;
pub mod diagnostics;
pub mod fish;
pub mod octopus;
pub mod path;
pub mod segment;
pub mod smoke;
pub mod sonar;
pub mod submarine;
pub mod vents;
pub mod fold;
pub mod poly;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

aoc_lib! { year = 2021 }
