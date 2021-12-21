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
        let height = input.lines().collect::<Vec<&str>>().len();

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
        let north = if y > 0 { self.tiles.get(y - 1).and_then(|r| r.get(x)).map(|h| *h) } else { None };
        let east = self.tiles.get(y).and_then(|r| r.get(x + 1)).map(|h| *h);
        let south = self.tiles.get(y + 1).and_then(|r| r.get(x)).map(|h| *h);
        let west = if x > 0 { self.tiles.get(y).and_then(|r| r.get(x - 1)).map(|h| *h) } else { None };
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
}

#[derive(Debug, PartialEq)]
pub struct Window {
    north: Option<u32>,
    east: Option<u32>,
    south: Option<u32>,
    west: Option<u32>,

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
}