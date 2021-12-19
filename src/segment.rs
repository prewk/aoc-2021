use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(PartialEq, Clone, Debug)]
pub struct Digit {
    pub letters: HashSet<Segment>,
}

impl From<&str> for Digit {
    fn from(input: &str) -> Self {
        let mut letters = HashSet::new();

        for char in input.chars() {
            letters.insert(match char {
                'a' => Segment::A,
                'b' => Segment::B,
                'c' => Segment::C,
                'd' => Segment::D,
                'e' => Segment::E,
                'f' => Segment::F,
                'g' => Segment::G,
                _ => panic!("Invalid input"),
            });
        }

        Digit { letters }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Eq for Segment {}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Wiring {
    wire_to: [Segment; 7],
}

impl Eq for Wiring {}

impl Wiring {
    pub fn get_mask(&self, segments: &HashSet<Segment>) -> u64 {
        let two = 2u64;
        let mut mask = 0u64;

        for w in 0..self.wire_to.len() {
            let wired = self.wire_to[w];
            let pow = u64::pow(two, w as u32);

            if segments.contains(&wired) {
                mask += pow;
            }
        }

        mask
    }

    pub fn get_index_for(&self, segment: &Segment) -> Option<usize> {
        self.wire_to.iter().position(|i| i == segment)
    }
}

fn get_all_possible() -> Vec<Wiring> {
    let all: [Segment; 7] = [
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::D,
        Segment::E,
        Segment::F,
        Segment::G,
    ];
    let mut combinations: Vec<Wiring> = vec![];

    for s1 in &all {
        for s2 in &all {
            for s3 in &all {
                for s4 in &all {
                    for s5 in &all {
                        for s6 in &all {
                            for s7 in &all {
                                let dupe = HashSet::from([*s1, *s2, *s3, *s4, *s5, *s6, *s7]);

                                if dupe.len() != 7 {
                                    continue;
                                }

                                combinations.push(Wiring {
                                    wire_to: [*s1, *s2, *s3, *s4, *s5, *s6, *s7],
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    combinations
}

fn filter_possible_using_data(
    possibles: &Vec<Wiring>,
    data: &HashSet<Segment>,
    relevant: &HashSet<Segment>,
) -> Vec<Wiring> {
    let mut filtered: Vec<Wiring> = vec![];
    let reference = Wiring { wire_to: [Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G] };

    for possible in possibles {
        //  aaaa
        // b    c
        // b    c
        //  dddd
        // e    f
        // e    f
        //  gggg
        //           ABCDEFG
        // possible: GFEDCBA
        // relevant: CF (1)
        //
        // Does putting AB in GFEDCBA light up CF?
        // GFEDCBA
        // .....BA
        // ..C..F.
        // No

        let data_mask = possible.get_mask(&data);
        let relevant_mask = reference.get_mask(&relevant);

        if data_mask == relevant_mask {
            filtered.push(possible.clone());
        }
    }

    filtered
}

fn filter_possible_by_rewiring(
    possibles: &Vec<Wiring>,
    digit: &Digit
) -> Vec<Wiring> {
    let mut filtered: HashSet<Wiring> = HashSet::new();

    for possible in possibles {
        if digit.wire(possible).is_some()  {
            filtered.insert(possible.clone());
        }
    }

    Vec::from_iter(filtered)
}

impl Digit {
    pub fn easy_guess(&self) -> Option<u64> {
        match self.letters.len() {
            2 => {
                return Some(1);
            }
            3 => {
                return Some(7);
            }
            4 => {
                return Some(4);
            }
            7 => {
                return Some(8);
            }
            _ => {}
        };

        return None;
    }

    pub fn wire(&self, wiring: &Wiring) -> Option<u64> {
        let mut segments: [bool; 7] = [false, false, false, false, false, false, false];

        for letter in &self.letters {
            let index = wiring.get_index_for(&letter)?;

            segments[index] = true;
        }

        match segments {
            [true, true, true, false, true, true, true] => Some(0),
            [false, false, true, false, false, true, false] => Some(1),
            [true, false, true, true, true, false, true] => Some(2),
            [true, false, true, true, false, true, true] => Some(3),
            [false, true, true, true, false, true, false] => Some(4),
            [true, true, false, true, false, true, true] => Some(5),
            [true, true, false, true, true, true, true] => Some(6),
            [true, false, true, false, false, true, false] => Some(7),
            [true, true, true, true, true, true, true] => Some(8),
            [true, true, true, true, false, true, true] => Some(9),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Entry {
    pub digits: Vec<Digit>,
    pub output: Vec<Digit>,
}

impl From<&str> for Entry {
    fn from(line: &str) -> Self {
        let parts: Vec<Vec<Digit>> = line
            .split(" | ")
            .map(|part| part.split(' ').map(Digit::from).collect())
            .collect();

        Entry {
            digits: parts.get(0).unwrap().clone(),
            output: parts.get(1).unwrap().clone(),
        }
    }
}

impl Entry {
    pub fn count_easily_guessed_outputs(&self) -> usize {
        self.output
            .iter()
            .map(|d| d.easy_guess())
            .filter_map(|s| s)
            .collect::<Vec<u64>>()
            .len()
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
        self.entries
            .iter()
            .map(|entry| {
                entry
                    .output
                    .iter()
                    .map(|d| d.easy_guess())
                    .filter_map(|s| s)
                    .collect::<Vec<u64>>()
                    .len()
            })
            .fold(0, |acc, len| acc + len)
    }

    pub fn count_real_output(&self) -> Option<u64> {
        let all_possible = get_all_possible();
        let ten = 10u64;
        let mut agg = 0;

        for entry in &self.entries {
            let mut possibles = all_possible.clone();
            let one_set = HashSet::from([Segment::C, Segment::F]);
            let four_set = HashSet::from([Segment::B, Segment::C, Segment::D, Segment::F]);
            let seven_set = HashSet::from([Segment::A, Segment::C, Segment::F]);

            // println!("possibles 1: {}", possibles.len());

            for digit in &entry.digits.iter().cloned().chain(entry.output.iter().cloned()).collect::<Vec<Digit>>() {
                match digit.easy_guess() {
                    Some(1) => {
                        possibles = filter_possible_using_data(&possibles, &digit.letters, &one_set);
                    },
                    Some(4) => {
                        possibles = filter_possible_using_data(&possibles, &digit.letters, &four_set);
                    },
                    Some(7) => {
                        possibles = filter_possible_using_data(&possibles, &digit.letters, &seven_set);
                    },
                    _ => {},
                }
            }

            for digit in &entry.output {
                possibles = filter_possible_by_rewiring(&possibles, digit);
            }

            // Expect at least one correct wiring to be left
            let correct_wiring = possibles.get(0)?;

            let output_count = entry.output.len();

            for i in 0..output_count {
                let digit = entry.output[i].wire(&correct_wiring)?;

                agg += digit * u64::pow(ten, (output_count - i - 1) as u32);
            }
        }

        Some(agg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let entry = Entry::from("acedgfb cdfbe ab | cdfeb");

        for c in [Segment::A, Segment::C, Segment::E, Segment::D, Segment::G, Segment::F, Segment::B] {
            assert!(entry.digits[0].letters.contains(&c));
        }
    }

    #[test]
    fn test_guess() {
        let entries = Entries::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
                                        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
                                        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
                                        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
                                        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
                                        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
                                        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
                                        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
                                        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
                                        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");

        assert_eq!(entries.count_easily_guessed_outputs(), 26);
    }

    #[test]
    fn test_mask() {
        let all = get_all_possible();

        assert_eq!(
            all[0],
            Wiring {
                wire_to: [
                    Segment::A,
                    Segment::B,
                    Segment::C,
                    Segment::D,
                    Segment::E,
                    Segment::F,
                    Segment::G
                ]
            }
        );

        let mask = all[0].get_mask(&HashSet::from([Segment::A, Segment::D, Segment::G]));

        assert_eq!(mask, 0b1001001);
    }

    #[test]
    fn test_wire() {
        let wiring1 = Wiring { wire_to: [Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G] };
        let wiring2 = Wiring { wire_to: [Segment::C, Segment::B, Segment::A, Segment::F, Segment::E, Segment::D, Segment::G] };

        assert_eq!(Digit::from("ad").wire(&wiring1), None);
        assert_eq!(Digit::from("ad").wire(&wiring2), Some(1));

        assert_eq!(Digit::from("cafeg").wire(&wiring1), None);
        assert_eq!(Digit::from("cafeg").wire(&wiring2), Some(2));

        assert_eq!(Digit::from("cafdg").wire(&wiring1), Some(3));
        assert_eq!(Digit::from("cafdg").wire(&wiring2), Some(3));

        assert_eq!(Digit::from("bafd").wire(&wiring1), None);
        assert_eq!(Digit::from("bafd").wire(&wiring2), Some(4));

        assert_eq!(Digit::from("cbfdg").wire(&wiring1), None);
        assert_eq!(Digit::from("cbfdg").wire(&wiring2), Some(5));

        assert_eq!(Digit::from("cbfedg").wire(&wiring1), None);
        assert_eq!(Digit::from("cbfedg").wire(&wiring2), Some(6));

        assert_eq!(Digit::from("acd").wire(&wiring1), None);
        assert_eq!(Digit::from("acd").wire(&wiring2), Some(7));

        assert_eq!(Digit::from("abcdefg").wire(&wiring1), Some(8));
        assert_eq!(Digit::from("abcdefg").wire(&wiring2), Some(8));

        assert_eq!(Digit::from("abcdfg").wire(&wiring1), Some(9));
        assert_eq!(Digit::from("abcdfg").wire(&wiring2), Some(9));

        assert_eq!(Digit::from("cbaedg").wire(&wiring1), None);
        assert_eq!(Digit::from("cbaedg").wire(&wiring2), Some(0));
    }

    #[test]
    fn test_filter_small() {
        let no = Wiring {
            wire_to: [
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
        };
        let yes = Wiring {
            wire_to: [
                Segment::C,
                Segment::F,
                Segment::A,
                Segment::D,
                Segment::E,
                Segment::B,
                Segment::G,
            ],
        };

        let filtered = filter_possible_using_data(
            &vec![no.clone(), yes.clone()],
            &HashSet::from([Segment::A, Segment::B]),
            &HashSet::from([Segment::C, Segment::F]),
        );

        assert_eq!(filtered, vec![yes.clone()])
    }

    #[test]
    fn test_filter_large() {
        let entries = Entries::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
                                        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
                                        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
                                        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
                                        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
                                        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
                                        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
                                        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
                                        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
                                        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");

        assert_eq!(entries.count_real_output(), Some(61229));
    }
}
