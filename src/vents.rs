use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Map {
    vents_without_diag: HashMap<(u64, u64), u64>,
    vents_with_diag: HashMap<(u64, u64), u64>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut vents_without_diag: HashMap<(u64, u64), u64> = HashMap::new();
        let mut vents_with_diag: HashMap<(u64, u64), u64> = HashMap::new();

        input
            .lines()
            .for_each(|line| {
                let coords: Vec<Vec<u64>> = line.split(" -> ").map(|coord| coord.split(',').map(|v| v.parse::<u64>()).filter_map(Result::ok).collect::<Vec<u64>>()).collect();
                let from_x = coords.get(0).expect("Invalid input").get(0).expect("Invalid input");
                let from_y = coords.get(0).expect("Invalid input").get(1).expect("Invalid input");
                let to_x = coords.get(1).expect("Invalid input").get(0).expect("Invalid input");
                let to_y = coords.get(1).expect("Invalid input").get(1).expect("Invalid input");

                if from_x == to_x {
                    // Vertical line
                    let range = match from_y < to_y {
                        true => *from_y..=*to_y,
                        false => *to_y..=*from_y,
                    };
                    for y in range {
                        *vents_without_diag.entry((*from_x, y)).or_insert(0) += 1;
                        *vents_with_diag.entry((*from_x, y)).or_insert(0) += 1;
                    }
                } else if from_y == to_y {
                    // Horizontal line
                    let range = match from_x < to_x {
                        true => *from_x..=*to_x,
                        false => *to_x..=*from_x,
                    };
                    for x in range {
                        *vents_without_diag.entry((x, *from_y)).or_insert(0) += 1;
                        *vents_with_diag.entry((x, *from_y)).or_insert(0) += 1;
                    }
                } else {
                    if to_x > from_x {
                        // ->
                        if to_y > from_y {
                            // \
                            //  v
                            for step in 0..(*to_y - *from_y) {
                                *vents_with_diag.entry((*from_x + step, *from_y + step)).or_insert(0) += 1;
                            }
                        } else {
                            //  ^
                            // /
                            for step in 0..(*from_y - *to_y) {
                                *vents_with_diag.entry((*from_x + step, *from_y - step)).or_insert(0) += 1;
                            }
                        }
                    } else {
                        // <-
                        if to_y > from_y {
                            //  /
                            // v
                            for step in 0..(*to_y - *from_y) {
                                *vents_with_diag.entry((*from_x - step, *from_y + step)).or_insert(0) += 1;
                            }
                        } else {
                            // ^
                            //  \
                            for step in 0..(*from_y - *to_y) {
                                *vents_with_diag.entry((*from_x - step, *from_y - step)).or_insert(0) += 1;
                            }
                        }
                    }
                }
            });

        Map {
            vents_without_diag,
            vents_with_diag,
        }
    }
}

impl Map {
    pub fn get_count_without_diagonals(&self) -> usize {
        self.vents_without_diag.values().filter(|v| v > &&1u64).count()
    }

    pub fn get_count_with_diagonals(&self) -> usize {
        self.vents_with_diag.values().filter(|v| v > &&1u64).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_diag() {
        let map = Map::from("0,9 -> 5,9\n\
                                         8,0 -> 0,8\n\
                                         9,4 -> 3,4\n\
                                         2,2 -> 2,1\n\
                                         7,0 -> 7,4\n\
                                         6,4 -> 2,0\n\
                                         0,9 -> 2,9\n\
                                         3,4 -> 1,4\n\
                                         0,0 -> 8,8\n\
                                         5,5 -> 8,2");

        assert_eq!(map.get_count_without_diagonals(), 5);
    }

    #[test]
    fn test_with_diag() {
        let map = Map::from("0,9 -> 5,9\n\
                                         8,0 -> 0,8\n\
                                         9,4 -> 3,4\n\
                                         2,2 -> 2,1\n\
                                         7,0 -> 7,4\n\
                                         6,4 -> 2,0\n\
                                         0,9 -> 2,9\n\
                                         3,4 -> 1,4\n\
                                         0,0 -> 8,8\n\
                                         5,5 -> 8,2");

        assert_eq!(map.get_count_with_diagonals(), 12);
    }
}