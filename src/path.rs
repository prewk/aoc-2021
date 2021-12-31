use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

impl From<&str> for Cave {
    fn from(cave: &str) -> Self {
        match cave {
            "start" => Cave::Start,
            "end" => Cave::End,
            cave => {
                if cave.to_uppercase() == cave {
                    Cave::Large(cave.to_string())
                } else {
                    Cave::Small(cave.to_string())
                }
            }
        }
    }
}

impl From<&Cave> for String {
    fn from(cave: &Cave) -> Self {
        match cave {
            Cave::Start => "start".to_string(),
            Cave::End => "end".to_string(),
            Cave::Small(c) => c.clone(),
            Cave::Large(c) => c.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Passage {
    pub left: Cave,
    pub right: Cave,
}

impl From<&str> for Passage {
    fn from(input: &str) -> Self {
        let parts = input.split('-').collect::<Vec<&str>>();
        let left = parts[0];
        let right = parts[1];

        Passage {
            left: Cave::from(left),
            right: Cave::from(right),
        }
    }
}

pub struct Map {
    lookup: HashMap<Cave, HashSet<Cave>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut lookup = HashMap::new();

        for passage in input.lines().map(Passage::from) {
            lookup
                .entry(passage.left.clone())
                .or_insert_with(HashSet::new)
                .insert(passage.right.clone());
            lookup
                .entry(passage.right.clone())
                .or_insert_with(HashSet::new)
                .insert(passage.left.clone());
        }

        Map { lookup }
    }
}

impl Map {
    fn visit(
        &self,
        subject: &Cave,
        visited_small: HashSet<String>,
        allow_twice: Option<&str>
    ) -> Result<Vec<Vec<Cave>>> {
        let mut paths = vec![];
        let caves = self.lookup.get(subject).context("Missing cave")?;

        'caves: for cave in caves {
            let mut visited_small = visited_small.clone();

            match cave {
                Cave::Start => continue 'caves,
                Cave::End => {
                    paths.push(vec![Cave::End]);
                }
                Cave::Small(c) => {
                    if visited_small.contains(c) {
                        continue 'caves;
                    }

                    if let Some(allowee) = allow_twice {
                        if &allowee.to_string() == c {
                            if visited_small.contains(&format!("{}-twice", c)) {
                                visited_small.insert(c.clone());
                            } else {
                                visited_small.insert(format!("{}-twice", c));
                            }
                        } else {
                            visited_small.insert(c.clone());
                        }
                    } else {
                        visited_small.insert(c.clone());
                    }


                    for children in self
                        .visit(cave, visited_small.clone(), allow_twice,)?
                        .iter_mut()
                    {
                        let mut this_and_children = vec![Cave::Small(c.clone())];

                        for child in children {
                            this_and_children.push(child.clone());
                        }

                        paths.push(this_and_children);
                    }
                }
                Cave::Large(c) => {
                    for children in self
                        .visit(cave, visited_small.clone(), allow_twice,)?
                        .iter_mut()
                    {
                        let mut this_and_children = vec![Cave::Large(c.clone())];

                        for child in children {
                            this_and_children.push(child.clone());
                        }

                        paths.push(this_and_children);
                    }
                }
            }
        }

        Ok(paths)
    }

    pub fn paths(&self) -> Result<HashSet<String>> {
        let mut paths = HashSet::new();

        for visit in self.visit(&Cave::Start, HashSet::new(), None)? {
            let mut this_and_children = vec![Cave::Start];

            this_and_children.extend(visit);

            paths.insert(path_to_string(&this_and_children));
        }

        Ok(paths)
    }

    pub fn paths_allow_twice(&self) -> Result<HashSet<String>> {
        let mut paths = HashSet::new();

        for allowee in self.lookup.keys() {
            if let Cave::Small(sm_cave) = allowee {
                for visit in self.visit(&Cave::Start, HashSet::new(), Some(sm_cave))? {
                    let mut this_and_children = vec![Cave::Start];

                    this_and_children.extend(visit);

                    paths.insert(path_to_string(&this_and_children));
                }
            }
        }

        Ok(paths)
    }
}

pub fn path_to_string(caves: &Vec<Cave>) -> String {
    caves
        .iter()
        .map(String::from)
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths_small() {
        let map = Map::from(
            "start-A\n\
            start-b\n\
            A-c\n\
            A-b\n\
            b-d\n\
            A-end\n\
            b-end",
        );

        let all = map.paths().unwrap();

        assert_eq!(all.len(), 10);
        assert!(all.contains("start,A,b,A,c,A,end"));
        assert!(all.contains("start,A,b,A,end"));
        assert!(all.contains("start,A,b,end"));
        assert!(all.contains("start,A,c,A,b,A,end"));
        assert!(all.contains("start,A,c,A,b,end"));
        assert!(all.contains("start,A,c,A,end"));
        assert!(all.contains("start,A,end"));
        assert!(all.contains("start,b,A,c,A,end"));
        assert!(all.contains("start,b,A,end"));
        assert!(all.contains("start,b,end"));
    }

    #[test]
    fn test_paths_small_part2() {
        let map = Map::from(
            "start-A\n\
            start-b\n\
            A-c\n\
            A-b\n\
            b-d\n\
            A-end\n\
            b-end",
        );

        let all = map.paths_allow_twice().unwrap();

        assert_eq!(all.len(), 36);
        assert!(all.contains("start,A,b,A,b,A,c,A,end"));
        assert!(all.contains("start,A,b,A,b,A,end"));
        assert!(all.contains("start,A,b,A,b,end"));
        assert!(all.contains("start,A,b,A,c,A,b,A,end"));
        assert!(all.contains("start,A,b,A,c,A,b,end"));
        assert!(all.contains("start,A,b,A,c,A,c,A,end"));
        assert!(all.contains("start,A,b,A,c,A,end"));
        assert!(all.contains("start,A,b,A,end"));
        assert!(all.contains("start,A,b,d,b,A,c,A,end"));
        assert!(all.contains("start,A,b,d,b,A,end"));
        assert!(all.contains("start,A,b,d,b,end"));
        assert!(all.contains("start,A,b,end"));
        assert!(all.contains("start,A,c,A,b,A,b,A,end"));
        assert!(all.contains("start,A,c,A,b,A,b,end"));
        assert!(all.contains("start,A,c,A,b,A,c,A,end"));
        assert!(all.contains("start,A,c,A,b,A,end"));
        assert!(all.contains("start,A,c,A,b,d,b,A,end"));
        assert!(all.contains("start,A,c,A,b,d,b,end"));
        assert!(all.contains("start,A,c,A,b,end"));
        assert!(all.contains("start,A,c,A,c,A,b,A,end"));
        assert!(all.contains("start,A,c,A,c,A,b,end"));
        assert!(all.contains("start,A,c,A,c,A,end"));
        assert!(all.contains("start,A,c,A,end"));
        assert!(all.contains("start,A,end"));
        assert!(all.contains("start,b,A,b,A,c,A,end"));
        assert!(all.contains("start,b,A,b,A,end"));
        assert!(all.contains("start,b,A,b,end"));
        assert!(all.contains("start,b,A,c,A,b,A,end"));
        assert!(all.contains("start,b,A,c,A,b,end"));
        assert!(all.contains("start,b,A,c,A,c,A,end"));
        assert!(all.contains("start,b,A,c,A,end"));
        assert!(all.contains("start,b,A,end"));
        assert!(all.contains("start,b,d,b,A,c,A,end"));
        assert!(all.contains("start,b,d,b,A,end"));
        assert!(all.contains("start,b,d,b,end"));
        assert!(all.contains("start,b,end"));
    }

    #[test]
    fn test_paths_medium() {
        let map = Map::from(
            "dc-end\n\
            HN-start\n\
            start-kj\n\
            dc-start\n\
            dc-HN\n\
            LN-dc\n\
            HN-end\n\
            kj-sa\n\
            kj-HN\n\
            kj-dc",
        );

        let all = map.paths().unwrap();

        assert_eq!(all.len(), 19);
        assert!(all.contains("start,HN,dc,HN,end"));
        assert!(all.contains("start,HN,dc,HN,kj,HN,end"));
        assert!(all.contains("start,HN,dc,end"));
        assert!(all.contains("start,HN,dc,kj,HN,end"));
        assert!(all.contains("start,HN,end"));
        assert!(all.contains("start,HN,kj,HN,dc,HN,end"));
        assert!(all.contains("start,HN,kj,HN,dc,end"));
        assert!(all.contains("start,HN,kj,HN,end"));
        assert!(all.contains("start,HN,kj,dc,HN,end"));
        assert!(all.contains("start,HN,kj,dc,end"));
        assert!(all.contains("start,dc,HN,end"));
        assert!(all.contains("start,dc,HN,kj,HN,end"));
        assert!(all.contains("start,dc,end"));
        assert!(all.contains("start,dc,kj,HN,end"));
        assert!(all.contains("start,kj,HN,dc,HN,end"));
        assert!(all.contains("start,kj,HN,dc,end"));
        assert!(all.contains("start,kj,HN,end"));
        assert!(all.contains("start,kj,dc,HN,end"));
        assert!(all.contains("start,kj,dc,end"));
    }

    #[test]
    fn test_paths_medium_part2() {
        let map = Map::from(
            "dc-end\n\
            HN-start\n\
            start-kj\n\
            dc-start\n\
            dc-HN\n\
            LN-dc\n\
            HN-end\n\
            kj-sa\n\
            kj-HN\n\
            kj-dc",
        );

        assert_eq!(map.paths_allow_twice().unwrap().len(), 103);
    }

    #[test]
    fn test_paths_large() {
        let map = Map::from(
            "fs-end\n\
            he-DX\n\
            fs-he\n\
            start-DX\n\
            pj-DX\n\
            end-zg\n\
            zg-sl\n\
            zg-pj\n\
            pj-he\n\
            RW-he\n\
            fs-DX\n\
            pj-RW\n\
            zg-RW\n\
            start-pj\n\
            he-WI\n\
            zg-he\n\
            pj-fs\n\
            start-RW",
        );

        let all = map.paths().unwrap();

        assert_eq!(all.len(), 226);
    }

    #[test]
    fn test_paths_large_part2() {
        let map = Map::from(
            "fs-end\n\
            he-DX\n\
            fs-he\n\
            start-DX\n\
            pj-DX\n\
            end-zg\n\
            zg-sl\n\
            zg-pj\n\
            pj-he\n\
            RW-he\n\
            fs-DX\n\
            pj-RW\n\
            zg-RW\n\
            start-pj\n\
            he-WI\n\
            zg-he\n\
            pj-fs\n\
            start-RW",
        );

        assert_eq!(map.paths_allow_twice().unwrap().len(), 3509);
    }
}
