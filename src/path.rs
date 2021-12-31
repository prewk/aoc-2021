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
                .or_insert(HashSet::new())
                .insert(passage.right.clone());
            lookup
                .entry(passage.right.clone())
                .or_insert(HashSet::new())
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
        depth: usize,
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

                    visited_small.insert(c.clone());

                    for children in self
                        .visit(cave, visited_small.clone(), depth + 1)?
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
                        .visit(cave, visited_small.clone(), depth + 1)?
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

    pub fn paths(&self) -> Result<Vec<Vec<Cave>>> {
        let mut paths = vec![];

        for visit in self.visit(&Cave::Start, HashSet::new(), 0)? {
            let mut this_and_children = vec![Cave::Start];

            this_and_children.extend(visit);

            paths.push(this_and_children);
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
    use std::iter::FromIterator;

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

        let all: HashSet<String> = HashSet::from_iter(
            map.paths()
                .unwrap()
                .iter()
                .map(path_to_string)
                .collect::<Vec<String>>(),
        );

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

        let all: HashSet<String> = HashSet::from_iter(
            map.paths()
                .unwrap()
                .iter()
                .map(path_to_string)
                .collect::<Vec<String>>(),
        );

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

        let all: HashSet<String> = HashSet::from_iter(
            map.paths()
                .unwrap()
                .iter()
                .map(path_to_string)
                .collect::<Vec<String>>(),
        );

        assert_eq!(all.len(), 226);
    }
}
