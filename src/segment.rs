// use std::fmt;
// use std::fmt::Formatter;
//
// #[derive(PartialEq, Copy, Clone, Debug)]
// pub enum Digit {
//     Zero,
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine,
// }
//
// impl Digit {
//     fn as_num(&self) -> u64 {
//         match *self {
//             Digit::Zero => 0,
//             Digit::One => 1,
//             Digit::Two => 2,
//             Digit::Three => 3,
//             Digit::Four => 4,
//             Digit::Five => 5,
//             Digit::Six => 6,
//             Digit::Seven => 7,
//             Digit::Eight => 8,
//             Digit::Nine => 9,
//         }
//     }
// }
//
// impl fmt::Display for Digit {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.as_num())
//     }
// }
//
// #[derive(PartialEq, Copy, Clone)]
// pub struct DigitDisplay {
//     //  -
//     //
//     //
//     a: bool,
//     // |
//     //
//     //
//     b: bool,
//     //   |
//     //
//     //
//     c: bool,
//     //
//     //  -
//     //
//     d: bool,
//     //
//     //
//     // |
//     e: bool,
//     //
//     //
//     //   |
//     f: bool,
//     //
//     //
//     //  -
//     g: bool,
// }
//
// impl From<&str> for DigitDisplay {
//     fn from(input: &str) -> Self {
//         let mut disp =  DigitDisplay {
//             a: false,
//             b: false,
//             c: false,
//             d: false,
//             e: false,
//             f: false,
//             g: false,
//         };
//
//         for char in input.chars() {
//             match char {
//                 'a' => { disp.a = true; },
//                 'b' => { disp.b = true; },
//                 'c' => { disp.c = true; },
//                 'd' => { disp.d = true; },
//                 'e' => { disp.e = true; },
//                 'f' => { disp.f = true; },
//                 'g' => { disp.g = true; },
//                 _ => panic!("Invalid char: {}", char),
//             }
//         }
//
//         disp
//     }
// }
//
// impl DigitDisplay {
//     pub fn get_digit(&self) -> Option<Digit> {
//         match (self.a, self.b, self.c, self.d, self.e, self.f, self.g) {
//             (true, true, true, false, true, true, true) => Some(Digit::Zero),
//             (false, false, true, false, false, true, false) => Some(Digit::One),
//             (true, false, true, true, true, false, true) => Some(Digit::Two),
//             (true, false, true, true, false, true, true) => Some(Digit::Three),
//             (false, true, true, true, false, true, false) => Some(Digit::Four),
//             (true, true, false, true, false, true, true) => Some(Digit::Five),
//             (true, true, false, true, true, true, true) => Some(Digit::Six),
//             (true, false, true, false, false, true, false) => Some(Digit::Seven),
//             (true, true, true, true, true, true, true) => Some(Digit::Eight),
//             (true, true, true, true, false, true, true) => Some(Digit::Nine),
//             _ => None,
//         }
//     }
// }
//
// pub fn parse_digit_displays(input: &str) -> Vec<DigitDisplay> {
//     input.split(' ').map(DigitDisplay::from).collect()
// }

use std::collections::HashSet;

#[derive(PartialEq, Clone, Debug)]
pub struct Digit {
    pub letters: HashSet<char>,
}

impl From<&str> for Digit {
    fn from(input: &str) -> Self {
        let mut letters = HashSet::new();

        for char in input.chars() {
            letters.insert(char);
        }

        Digit {
            letters
        }
    }
}

impl Digit {
    pub fn easy_guess(&self) -> Option<u8> {
        match self.letters.len() {
            2 => { return Some(1); }
            3 => { return Some(7); }
            4 => { return Some(4); }
            7 => { return Some(8); }
            _ => {}
        };

        return None;
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Entry {
    pub digits: Vec<Digit>,
    pub output: Vec<Digit>,
}

impl From<&str> for Entry {
    fn from(line: &str) -> Self {
        let parts: Vec<Vec<Digit>> = line.split(" | ").map(|part| part.split(' ').map(Digit::from).collect()).collect();

        Entry {
            digits: parts.get(0).unwrap().clone(),
            output: parts.get(1).unwrap().clone(),
        }
    }
}

impl Entry {
    pub fn count_easily_guessed_outputs(&self) -> usize {
        self.output.iter().map(|d| d.easy_guess()).filter_map(|s| s).collect::<Vec<u8>>().len()
    }
}

pub struct Entries {
    entries: Vec<Entry>,
}

impl From<&str> for Entries {
    fn from(input: &str) -> Self {
        Entries {
            entries: input.lines().map(Entry::from).collect(),
        }
    }
}

impl Entries {
    pub fn count_easily_guessed_outputs(&self) -> usize {
        self.entries.iter().map(|entry| entry.output.iter().map(|d| d.easy_guess()).filter_map(|s| s).collect::<Vec<u8>>().len()).fold(0, |acc, len| { acc + len })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let entry = Entry::from("acedgfb cdfbe ab | cdfeb");

        for c in "acedgfb".chars() {
            assert!(entry.digits[0].letters.contains(&c));
        }

        for c in "cdfbe".chars() {
            assert!(entry.digits[1].letters.contains(&c));
        }

        for c in "ab".chars() {
            assert!(entry.digits[2].letters.contains(&c));
        }

        for c in "cdfeb".chars() {
            assert!(entry.output[0].letters.contains(&c));
        }
    }

    #[test]
    fn test_guess() {
        let entry = Entries::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
                                        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
                                        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
                                        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
                                        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
                                        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
                                        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
                                        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
                                        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
                                        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");

        assert_eq!(entry.count_easily_guessed_outputs(), 26);
    }
}