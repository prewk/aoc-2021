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
    pub template: Vec<char>,
    pub rules: Vec<Rule>,
}

impl From<&str> for Poly {
    fn from(input: &str) -> Self {
        Poly {
            template: input.lines().next().expect("Invalid input (1)").chars().collect(),
            rules: input.lines().skip(2).map(Rule::from).collect(),
        }
    }
}

impl Iterator for Poly {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Vec<char> = vec![];

        for i in 0..(self.template.len() - 1) {
            let left = *self.template.get(i)?;
            let right = *self.template.get(i + 1)?;

            next.push(left);
            if let Some(rule) = self.rules.iter().find(|rule| rule.left == left && rule.right == right) {
                next.push(rule.insertee);
            }
        }

        next.push(*self.template.last()?);

        self.template = next.clone();

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

        for c in &self.template {
            *map.entry(*c).or_insert(0) += 1;
        }

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

    pub fn part1(&self) -> Option<u64> {
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

        assert_eq!(poly.template, "NNCB".chars().collect::<Vec<char>>());
        assert_eq!(poly.common().unwrap().least_cnt, 1);
        assert_eq!(poly.common().unwrap().most_cnt, 2);

        poly.next().unwrap();
        assert_eq!(poly.template, "NCNBCHB".chars().collect::<Vec<char>>());
        assert_eq!(poly.common().unwrap().least_cnt, 1);
        assert_eq!(poly.common().unwrap().most_cnt, 2);

        poly.next().unwrap();
        assert_eq!(poly.template, "NBCCNBBBCBHCB".chars().collect::<Vec<char>>());
        assert_eq!(poly.common().unwrap().least_cnt, 1);
        assert_eq!(poly.common().unwrap().most_cnt, 6);

        poly.next().unwrap();
        assert_eq!(poly.template, "NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect::<Vec<char>>());
        assert_eq!(poly.common().unwrap().least_cnt, 4);
        assert_eq!(poly.common().unwrap().most_cnt, 11);

        poly.next().unwrap();
        assert_eq!(poly.template, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars().collect::<Vec<char>>());
        assert_eq!(poly.common().unwrap().least_cnt, 5);
        assert_eq!(poly.common().unwrap().most_cnt, 23);

        poly.next().unwrap();
        poly.next().unwrap();
        poly.next().unwrap();
        poly.next().unwrap();
        poly.next().unwrap();
        poly.next().unwrap();

        assert_eq!(poly.common(), Some(PolyLetterCount {
            least: 'H',
            least_cnt: 161,
            most: 'B',
            most_cnt: 1749,
        }));
    }
}