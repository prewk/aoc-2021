use std::collections::{HashMap};

#[derive(Debug, PartialEq)]
pub struct Map {
    tiles: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

pub fn risk_level(height: u32) -> u32 {
    height + 1
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut width = 0;
        let height = input.lines().count();

        Map {
            tiles: input.lines().map(|line| {
                width = line.len();

                line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>()
            }).collect(),
            width,
            height,
        }
    }
}

impl Map {
    pub fn peek(&self, x: usize, y: usize) -> Option<Window> {
        let north = if y > 0 { self.tiles.get(y - 1).and_then(|r| r.get(x)).copied() } else { None };
        let east = self.tiles.get(y).and_then(|r| r.get(x + 1)).copied();
        let south = self.tiles.get(y + 1).and_then(|r| r.get(x)).copied();
        let west = if x > 0 { self.tiles.get(y).and_then(|r| r.get(x - 1)).copied() } else { None };
        let center = *self.tiles.get(y).and_then(|r| r.get(x))?;

        Some(Window {
            north,
            east,
            south,
            west,
            center,
        })
    }

    pub fn find_low_points(&self) -> Vec<Window> {
        let mut low_points: Vec<Window> = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let window = self.peek(x, y).expect("Missing coordinate!");

                if window.is_low_point() {
                    low_points.push(window);
                }
            }
        }

        low_points
    }

    pub fn detect_basin(&self, x: usize, y: usize, inc_visited: &HashMap<(usize, usize), bool>) -> HashMap<(usize, usize), bool> {
        let window = self.peek(x, y).expect("Invalid coords");

        let mut visited = inc_visited.clone();

        if window.center == 9 {
            visited.insert((x, y), false);

            return visited;
        } else {
            visited.insert((x, y), true);
        }

        for (dir, pos) in [
            (&window.north, if y > 0 { Some((x, y - 1)) } else { None }),
            (&window.east, Some((x + 1, y))),
            (&window.south, Some((x, y + 1))),
            (&window.west, if x > 0 { Some((x - 1, y)) } else { None })
        ] {
            if let Some((it_x, it_y)) = pos {
                if let Some(n) = *dir {
                    if visited.contains_key(&(it_x, it_y)) {
                        continue;
                    }

                    if n < 9 {
                        for ((rec_x, rec_y), in_basin) in self.detect_basin(it_x, it_y, &visited) {
                            visited.insert((rec_x, rec_y), in_basin);
                        }
                    } else {
                        visited.insert((it_x, it_y), false);
                    }
                }
            }
        }

        visited
    }

    pub fn find_basins(&self) -> Vec<usize> {
        let mut basins = vec![];
        let mut visited = HashMap::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if visited.contains_key(&(x, y)) { continue; }

                let detected = self.detect_basin(x, y, &HashMap::new());

                let mut total = 0;

                for ((d_x, d_y), in_basin) in &detected {
                    visited.insert((*d_x, *d_y), *in_basin);

                    if *in_basin {
                        total += 1;
                    }
                }

                if total > 0 {
                    basins.push(total);
                }
            }
        }

        basins
    }

    pub fn mult_three_largest_basins(&self) -> usize {
        let mut basins = self.find_basins();

        basins.sort_unstable();

        basins.iter().rev().take(3).fold(1, |prod, b| b * prod)
    }
}

#[derive(Debug, PartialEq)]
pub struct Window {
    pub north: Option<u32>,
    pub east: Option<u32>,
    pub south: Option<u32>,
    pub west: Option<u32>,

    pub center: u32,
}

impl Window {
    pub fn is_low_point(&self) -> bool {
        if self.center >= self.north.unwrap_or(u32::MAX) { return false; }
        if self.center >= self.east.unwrap_or(u32::MAX) { return false; }
        if self.center >= self.south.unwrap_or(u32::MAX) { return false; }
        if self.center >= self.west.unwrap_or(u32::MAX) { return false; }

        true
    }
}

pub fn risk_sum(map: &Map) -> u32 {
    let low_points = map.find_low_points();

    low_points.iter().map(|w| risk_level(w.center)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_sum() {
        let map = Map::from("2199943210\n\
                3987894921\n\
                9856789892\n\
                8767896789\n\
                9899965678");

        assert_eq!(risk_sum(&map), 15);
    }

    #[test]
    fn test_basin_detector() {
        let map = Map::from("2199943210\n\
                3987894921\n\
                9856789892\n\
                8767896789\n\
                9899965678");

        let possible = map.detect_basin(0, 0, &HashMap::new());

        assert_eq!(possible.len(), 6);
        assert_eq!(possible.get(&(0, 0)), Some(&true));
        assert_eq!(possible.get(&(1, 0)), Some(&true));
        assert_eq!(possible.get(&(2, 0)), Some(&false));
        assert_eq!(possible.get(&(0, 1)), Some(&true));
        assert_eq!(possible.get(&(1, 1)), Some(&false));
        assert_eq!(possible.get(&(0, 2)), Some(&false));
    }

    #[test]
    fn test_find_basins() {
        let map = Map::from("2199943210\n\
                3987894921\n\
                9856789892\n\
                8767896789\n\
                9899965678");

        assert_eq!(map.find_basins(), vec![3, 9, 14, 9])
    }

    #[test]
    fn test_mult_three_largest_basins() {
        let map = Map::from("2199943210\n\
                3987894921\n\
                9856789892\n\
                8767896789\n\
                9899965678");

        assert_eq!(map.mult_three_largest_basins(), 1134);
    }
}