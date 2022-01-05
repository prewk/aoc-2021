use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Rule {
    left: char,
    right: char,
    insertee: char,
}

impl From<&str> for Rule {
    fn from(line: &str) -> Self {
        let parts = line.split(" -> ").collect::<Vec<&str>>();

        Rule {
            left: parts[0].chars().next().expect("Invalid input (2)"),
            right: parts[0].chars().nth(1).expect("Invalid input (3)"),
            insertee: parts[1].chars().next().expect("Invalid input (4)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Poly {
    pub rules: Vec<Rule>,
    pub stats: HashMap<[char; 2], u64>,
    pub template: Vec<char>,
}

impl From<&str> for Poly {
    fn from(input: &str) -> Self {
        let template = input.lines().next().expect("Invalid input (1)").chars().collect::<Vec<char>>();

        let mut stats: HashMap<[char; 2], u64> = HashMap::new();

        for i in 0..(template.len() - 1) {
            if let (Some(left), Some(right)) = (template.get(i), template.get(i + 1)) {
                *stats.entry([*left, *right]).or_insert(0) += 1;
            }
        }

        Poly {
            rules: input.lines().skip(2).map(Rule::from).collect(),
            stats,
            template,
        }
    }
}

impl Iterator for Poly {
    type Item = HashMap<[char; 2], u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = HashMap::new();

        for rule in &self.rules {
            if let Some(stat) = self.stats.get(&[rule.left, rule.right]) {
                *next.entry([rule.left, rule.insertee]).or_insert(0) += stat;
                *next.entry([rule.insertee, rule.right]).or_insert(0) += stat;
            }
        }

        self.stats = next.clone();

        Some(next)
    }
}

#[derive(Debug, PartialEq)]
pub struct PolyLetterCount {
    pub most: char,
    pub most_cnt: u64,
    pub least: char,
    pub least_cnt: u64
}

impl Poly {
    pub fn common(&self) -> Option<PolyLetterCount> {
        let mut map: HashMap<char, u64> = HashMap::new();

        for (key, value) in &self.stats {
            let left = key[0];

            *map.entry(left).or_insert(0) += *value;
        }

        let last = *self.template.last()?;
        *map.entry(last).or_insert(0) += 1;

        let mut most = map.keys().next()?;
        let mut least = map.keys().next()?;

        for (key, value) in &map {
            let most_cnt = map.get(most)?;
            let least_cnt = map.get(least)?;

            if value > most_cnt {
                most = key;
            }
            if value < least_cnt {
                least = key;
            }
        }

        Some(PolyLetterCount {
            most: *most,
            most_cnt: *map.get(most)?,
            least: *least,
            least_cnt: *map.get(least)?,
        })
    }

    pub fn puzzle_output(&self) -> Option<u64> {
        let common = self.common()?;

        Some(common.most_cnt - common.least_cnt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly() {
        let mut poly = Poly::from("NNCB\n\
                                    \n\
                                    CH -> B\n\
                                    HH -> N\n\
                                    CB -> H\n\
                                    NH -> C\n\
                                    HB -> C\n\
                                    HC -> B\n\
                                    HN -> C\n\
                                    NN -> C\n\
                                    BH -> H\n\
                                    NC -> B\n\
                                    NB -> B\n\
                                    BN -> B\n\
                                    BB -> N\n\
                                    BC -> B\n\
                                    CC -> N\n\
                                    CN -> C");

        for _ in 0..10 {
            poly.next().unwrap();
        }

        assert_eq!(poly.common().unwrap().least_cnt, 161);
        assert_eq!(poly.common().unwrap().most_cnt, 1749);
        assert_eq!(poly.puzzle_output(), Some(1588));
    }
}