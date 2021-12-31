use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Map {
    pub octopi: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
    pub flash_count: usize,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut octopi = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in input.lines() {
            width = line.len();

            octopi.push(
                line.chars()
                    .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
                    .collect(),
            );

            height += 1;
        }

        Map {
            octopi,
            width,
            height,
            flash_count: 0,
        }
    }
}

impl Map {
    pub fn tick(&self) -> Map {
        // +1 Energy
        let mut wip: Vec<Vec<u8>> = self
            .octopi
            .iter()
            .map(|row| row.iter().map(|octopus| octopus + 1).collect())
            .collect();

        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        let mut flash_one = || {
            for y in 0..self.height {
                for x in 0..self.width {
                    let value = wip[y][x];

                    if value > 9 && !flashed.contains(&(x, y)) {
                        if y > 0 && x > 0 && !flashed.contains(&(x - 1, y - 1)) {
                            wip[y - 1][x - 1] += 1;
                        }
                        if y > 0 && !flashed.contains(&(x, y - 1)) {
                            wip[y - 1][x] += 1;
                        }
                        if y > 0 && x < self.width - 1 && !flashed.contains(&(x + 1, y - 1)) {
                            wip[y - 1][x + 1] += 1;
                        }

                        if x > 0 && !flashed.contains(&(x - 1, y)) {
                            wip[y][x - 1] += 1;
                        }
                        if x < self.width - 1 && !flashed.contains(&(x + 1, y)) {
                            wip[y][x + 1] += 1;
                        }

                        if x > 0 && y < self.height - 1 && !flashed.contains(&(x - 1, y + 1)) {
                            wip[y + 1][x - 1] += 1;
                        }
                        if y < self.height - 1 && !flashed.contains(&(x, y + 1)) {
                            wip[y + 1][x] += 1;
                        }
                        if x < self.width - 1
                            && y < self.height - 1
                            && !flashed.contains(&(x + 1, y + 1))
                        {
                            wip[y + 1][x + 1] += 1;
                        }

                        flashed.insert((x, y));

                        wip[y][x] = 0;

                        return true;
                    }
                }
            }

            false
        };

        let mut run = true;

        while run {
            run = flash_one();
        }

        Map {
            octopi: wip,
            width: self.width,
            height: self.height,
            flash_count: self.flash_count + flashed.len(),
        }
    }
}

pub fn find_sync_flash_step(map: &Map) -> usize {
    let mut step = map.clone();
    let mut i = 0;

    while step
        .octopi
        .iter()
        .flatten()
        .map(|o| *o as usize)
        .sum::<usize>()
        > 0
    {
        step = step.tick();

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let step0 = Map::from(
            "11111\n\
                                19991\n\
                                19191\n\
                                19991\n\
                                11111",
        );

        let step1 = step0.tick();

        assert_eq!(
            step1.octopi,
            vec![
                vec![3, 4, 5, 4, 3],
                vec![4, 0, 0, 0, 4],
                vec![5, 0, 0, 0, 5],
                vec![4, 0, 0, 0, 4],
                vec![3, 4, 5, 4, 3],
            ]
        );

        let step2 = step1.tick();

        assert_eq!(
            step2.octopi,
            vec![
                vec![4, 5, 6, 5, 4],
                vec![5, 1, 1, 1, 5],
                vec![6, 1, 1, 1, 6],
                vec![5, 1, 1, 1, 5],
                vec![4, 5, 6, 5, 4],
            ]
        );
    }

    #[test]
    fn test_big() {
        let step0 = Map::from(
            "5483143223\n\
                                    2745854711\n\
                                    5264556173\n\
                                    6141336146\n\
                                    6357385478\n\
                                    4167524645\n\
                                    2176841721\n\
                                    6882881134\n\
                                    4846848554\n\
                                    5283751526",
        );

        let step1 = step0.tick();

        assert_eq!(
            step1.octopi,
            vec![
                vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
            ]
        );

        let mut step = step1;
        for _ in 0..9 {
            step = step.tick();
        }

        assert_eq!(
            step.octopi,
            vec![
                vec![0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
                vec![0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
                vec![0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
                vec![0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
                vec![0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
                vec![0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
                vec![0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
                vec![5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
                vec![0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
                vec![0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
            ]
        );

        for _ in 0..90 {
            step = step.tick();
        }

        assert_eq!(
            step.octopi,
            vec![
                vec![0, 3, 9, 7, 6, 6, 6, 8, 6, 6],
                vec![0, 7, 4, 9, 7, 6, 6, 9, 1, 8],
                vec![0, 0, 5, 3, 9, 7, 6, 9, 3, 3],
                vec![0, 0, 0, 4, 2, 9, 7, 8, 2, 2],
                vec![0, 0, 0, 4, 2, 2, 9, 8, 9, 2],
                vec![0, 0, 5, 3, 2, 2, 2, 8, 7, 7],
                vec![0, 5, 3, 2, 2, 2, 2, 9, 6, 6],
                vec![9, 3, 2, 2, 2, 2, 8, 9, 6, 6],
                vec![7, 9, 2, 2, 2, 8, 6, 8, 6, 6],
                vec![6, 7, 8, 9, 9, 9, 8, 7, 6, 6],
            ]
        );

        assert_eq!(step.flash_count, 1656);
    }

    #[test]
    fn test_sync_flash() {
        let map = Map::from(
            "5483143223\n\
                                    2745854711\n\
                                    5264556173\n\
                                    6141336146\n\
                                    6357385478\n\
                                    4167524645\n\
                                    2176841721\n\
                                    6882881134\n\
                                    4846848554\n\
                                    5283751526",
        );

        assert_eq!(find_sync_flash_step(&map), 195);
    }
}
