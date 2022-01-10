use crate::pathfinding::{astar_search, Map, Node, Pos};

pub struct Chitons {
    map: Map,
    width: usize,
    height: usize,
}

impl From<&str> for Chitons {
    fn from(input: &str) -> Self {
        Chitons {
            map: Map::from(&input
                .lines()
                .enumerate()
                .map(|(y, line)|
                    line.chars().enumerate().map(|(x, c)| Node { cost: c.to_digit(10).expect("Invalid digit") as i64, position: Pos { x: x as i64, y: y as i64 } }).collect::<Vec<Node>>()
                )
                .flatten()
                .collect::<Vec<Node>>()
            ),
            width: input.lines().next().expect("Invalid line").len(),
            height: input.lines().count(),
        }
    }
}

impl Chitons {
    pub fn part1(&self) -> Option<i64> {
        let path = astar_search(Pos { x: 0, y: 0, }, Pos { x: self.width as i64 - 1, y: self.height as i64 - 1}, &self.map, None)?;

        Some(path.iter().skip(1).map(|p| self.map.nodes.get(p)).filter_map(|o| o).map(|n| n.cost).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chitons() {
        let chitons = Chitons::from("1163751742\n\
                    1381373672\n\
                    2136511328\n\
                    3694931569\n\
                    7463417111\n\
                    1319128137\n\
                    1359912421\n\
                    3125421639\n\
                    1293138521\n\
                    2311944581");

        assert_eq!(chitons.part1(), Some(40));
    }
}