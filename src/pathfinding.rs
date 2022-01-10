use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::collections::{VecDeque};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialOrd, PartialEq, Ord)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Node {
    pub cost: i64,
    pub position: Pos,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Map {
    pub nodes: HashMap<Pos, Node>,
}

impl From<&Vec<Node>> for Map {
    fn from(input: &Vec<Node>) -> Self {
        let mut nodes = HashMap::new();

        for node in input {
            nodes.insert(node.position, *node);
        }

        Map {
            nodes,
        }
    }
}

impl Pos {
    pub fn neighbours(&self, map: &Map, threshold: Option<i64>) -> Vec<Pos> {
        vec![
            Pos { x: self.x, y: self.y - 1 },
            Pos { x: self.x - 1, y: self.y },
            Pos { x: self.x + 1, y: self.y },
            Pos { x: self.x, y: self.y + 1 },
        ].iter().filter(|pos| {
            match (threshold, map.nodes.get(pos)) {
                (None, Some(_)) => true,
                (Some(threshold), Some(node)) => node.cost <= threshold,
                _ => false,
            }
        }).copied().collect()
    }
}

pub fn came_from_to_path(goal: Pos, came_from: &HashMap<Pos, Option<Pos>>) -> Option<Vec<Pos>> {
    let mut path = vec![goal];
    let mut next = goal;

    while let Some(prev) = came_from.get(&next)? {
        path.push(*prev);

        next = *prev;
    }

    path.reverse();

    Some(path)
}

pub fn breadth_first_search(start: Pos, goal: Pos, map: &Map, threshold: Option<i64>) -> Option<Vec<Pos>> {
    let mut frontier = VecDeque::from([start]);
    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    came_from.insert(start, None);

    while let Some(current) = frontier.pop_front() {
        if current == goal { break; }

        for next in current.neighbours(map, threshold) {
            if !came_from.contains_key(&next) {
                frontier.push_back(next);
                came_from.insert(next, Some(current));
            }
        }
    }

    came_from_to_path(goal, &came_from)
}

pub fn dijkstras_search(start: Pos, goal: Pos, map: &Map, threshold: Option<i64>) -> Option<Vec<Pos>> {
    let mut frontier = BinaryHeap::from([*map.nodes.get(&start)?]);
    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    let mut cost_so_far: HashMap<Pos, i64> = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    while let Some(current) = frontier.pop() {
        if current.position == goal { break; }

        for next in current.position.neighbours(map, threshold) {
            let new_cost = cost_so_far.get(&current.position)? + map.nodes.get(&next)?.cost;

            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next)? {
                cost_so_far.insert(next, new_cost);
                frontier.push(Node { position: next, cost: new_cost });
                came_from.insert(next, Some(current.position));
            }
        }
    }

    came_from_to_path(goal, &came_from)
}

pub fn heuristic(a: Pos, b: Pos) -> i64 {
    (a.x - b.x).abs() - (a.y - b.y).abs()
}

pub fn greedy_best_first_search(start: Pos, goal: Pos, map: &Map, threshold: Option<i64>) -> Option<Vec<Pos>> {
    let mut frontier = BinaryHeap::from([*map.nodes.get(&start)?]);
    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    came_from.insert(start, None);

    while let Some(current) = frontier.pop() {
        if current.position == goal { break; }

        for next in current.position.neighbours(map, threshold) {
            if !came_from.contains_key(&next) {
                frontier.push(Node { position: next, cost: heuristic(goal, next) });
                came_from.insert(next, Some(current.position));
            }
        }
    }

    came_from_to_path(goal, &came_from)
}

pub fn astar_search(start: Pos, goal: Pos, map: &Map, threshold: Option<i64>) -> Option<Vec<Pos>> {
    let mut frontier = BinaryHeap::from([*map.nodes.get(&start)?]);
    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    let mut cost_so_far: HashMap<Pos, i64> = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    while let Some(current) = frontier.pop() {
        if current.position == goal { break; }

        for next in current.position.neighbours(map, threshold) {
            let new_cost = cost_so_far.get(&current.position)? + map.nodes.get(&next)?.cost;

            if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next)? {
                cost_so_far.insert(next, new_cost);
                frontier.push(Node { position: next, cost: new_cost + heuristic(goal, next) });
                came_from.insert(next, Some(current.position));
            }
        }
    }

    came_from_to_path(goal, &came_from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadth() {
        let path = breadth_first_search(Pos { x: 0, y: 0 }, Pos { x: 3, y: 3 }, &Map::from(&vec![
            Node { cost: 0, position: Pos { x: 0, y: 0 } }, Node { cost: 0, position: Pos { x: 1, y: 0 } }, Node { cost: 1, position: Pos { x: 2, y: 0 } }, Node { cost: 1, position: Pos { x: 3, y: 0 } },
            Node { cost: 0, position: Pos { x: 0, y: 1 } }, Node { cost: 0, position: Pos { x: 1, y: 1 } }, Node { cost: 1, position: Pos { x: 2, y: 1 } }, Node { cost: 1, position: Pos { x: 3, y: 1 } },
            Node { cost: 0, position: Pos { x: 0, y: 2 } }, Node { cost: 0, position: Pos { x: 1, y: 2 } }, Node { cost: 1, position: Pos { x: 2, y: 2 } }, Node { cost: 1, position: Pos { x: 3, y: 2 } },
            Node { cost: 0, position: Pos { x: 0, y: 3 } }, Node { cost: 0, position: Pos { x: 1, y: 3 } }, Node { cost: 0, position: Pos { x: 2, y: 3 } }, Node { cost: 0, position: Pos { x: 3, y: 3 } },
        ]), Some(0));

        assert_eq!(path, Some(vec![
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 1, y: 1 },
            Pos { x: 1, y: 2 },
            Pos { x: 1, y: 3 },
            Pos { x: 2, y: 3 },
            Pos { x: 3, y: 3 },
        ]));
    }

    #[test]
    fn test_dijkstras_search() {
        let path = dijkstras_search(Pos { x: 0, y: 0 }, Pos { x: 3, y: 3 }, &Map::from(&vec![
            Node { cost: 0, position: Pos { x: 0, y: 0 } }, Node { cost: 2, position: Pos { x: 1, y: 0 } }, Node { cost: 1, position: Pos { x: 2, y: 0 } }, Node { cost: 1, position: Pos { x: 3, y: 0 } },
            Node { cost: 1, position: Pos { x: 0, y: 1 } }, Node { cost: 0, position: Pos { x: 1, y: 1 } }, Node { cost: 1, position: Pos { x: 2, y: 1 } }, Node { cost: 1, position: Pos { x: 3, y: 1 } },
            Node { cost: 0, position: Pos { x: 0, y: 2 } }, Node { cost: 3, position: Pos { x: 1, y: 2 } }, Node { cost: 1, position: Pos { x: 2, y: 2 } }, Node { cost: 1, position: Pos { x: 3, y: 2 } },
            Node { cost: 0, position: Pos { x: 0, y: 3 } }, Node { cost: 0, position: Pos { x: 1, y: 3 } }, Node { cost: 0, position: Pos { x: 2, y: 3 } }, Node { cost: 0, position: Pos { x: 3, y: 3 } },
        ]), None);

        assert_eq!(path, Some(vec![
            Pos { x: 0, y: 0 },
            Pos { x: 0, y: 1 },
            Pos { x: 0, y: 2 },
            Pos { x: 0, y: 3 },
            Pos { x: 1, y: 3 },
            Pos { x: 2, y: 3 },
            Pos { x: 3, y: 3 },
        ]));
    }

    #[test]
    fn test_greedy_best_first_search() {
        let path = greedy_best_first_search(Pos { x: 0, y: 0 }, Pos { x: 3, y: 3 }, &Map::from(&vec![
            Node { cost: 0, position: Pos { x: 0, y: 0 } }, Node { cost: 2, position: Pos { x: 1, y: 0 } }, Node { cost: 1, position: Pos { x: 2, y: 0 } }, Node { cost: 1, position: Pos { x: 3, y: 0 } },
            Node { cost: 1, position: Pos { x: 0, y: 1 } }, Node { cost: 0, position: Pos { x: 1, y: 1 } }, Node { cost: 1, position: Pos { x: 2, y: 1 } }, Node { cost: 1, position: Pos { x: 3, y: 1 } },
            Node { cost: 0, position: Pos { x: 0, y: 2 } }, Node { cost: 3, position: Pos { x: 1, y: 2 } }, Node { cost: 1, position: Pos { x: 2, y: 2 } }, Node { cost: 1, position: Pos { x: 3, y: 2 } },
            Node { cost: 0, position: Pos { x: 0, y: 3 } }, Node { cost: 0, position: Pos { x: 1, y: 3 } }, Node { cost: 0, position: Pos { x: 2, y: 3 } }, Node { cost: 0, position: Pos { x: 3, y: 3 } },
        ]), None);

        assert_eq!(path, Some(vec![
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 2, y: 0 },
            Pos { x: 3, y: 0 },
            Pos { x: 3, y: 1 },
            Pos { x: 3, y: 2 },
            Pos { x: 3, y: 3 }
        ]));
    }


    #[test]
    fn test_astar_search() {
        let path = astar_search(Pos { x: 0, y: 0 }, Pos { x: 3, y: 3 }, &Map::from(&vec![
            Node { cost: 0, position: Pos { x: 0, y: 0 } }, Node { cost: 2, position: Pos { x: 1, y: 0 } }, Node { cost: 1, position: Pos { x: 2, y: 0 } }, Node { cost: 1, position: Pos { x: 3, y: 0 } },
            Node { cost: 1, position: Pos { x: 0, y: 1 } }, Node { cost: 0, position: Pos { x: 1, y: 1 } }, Node { cost: 1, position: Pos { x: 2, y: 1 } }, Node { cost: 1, position: Pos { x: 3, y: 1 } },
            Node { cost: 0, position: Pos { x: 0, y: 2 } }, Node { cost: 3, position: Pos { x: 1, y: 2 } }, Node { cost: 1, position: Pos { x: 2, y: 2 } }, Node { cost: 1, position: Pos { x: 3, y: 2 } },
            Node { cost: 0, position: Pos { x: 0, y: 3 } }, Node { cost: 0, position: Pos { x: 1, y: 3 } }, Node { cost: 0, position: Pos { x: 2, y: 3 } }, Node { cost: 0, position: Pos { x: 3, y: 3 } },
        ]), None);

        assert_eq!(path, Some(vec![
            Pos { x: 0, y: 0 },
            Pos { x: 0, y: 1 },
            Pos { x: 1, y: 1 },
            Pos { x: 2, y: 1 },
            Pos { x: 3, y: 1 },
            Pos { x: 3, y: 2 },
            Pos { x: 3, y: 3 }
        ]));
    }

}