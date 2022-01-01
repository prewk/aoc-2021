use anyhow::{Result};
use std::collections::{HashSet};

#[derive(Debug, Clone)]
pub enum Fold {
    Horizontal(i64),
    Vertical(i64),
}

#[derive(Debug, Clone)]
pub struct Instructions {
    dots: HashSet<(i64, i64)>,
    pub folds: Vec<Fold>,
}

#[derive(Debug, Clone)]
pub struct Paper {
    pub rows: Vec<Vec<bool>>,
}

impl From<&str> for Instructions {
    fn from(input: &str) -> Self {
        let mut dots = HashSet::new();
        let mut folds = vec![];

        for line in input.lines() {
            if line.starts_with("fold along y=") {
                folds.push(
                    Fold::Horizontal(line.split('=').collect::<Vec<&str>>().get(1).expect("Invalid y fold").parse::<i64>().expect("Invalid y fold parse"))
                );
            } else if line.starts_with("fold along x=") {
                folds.push(
                    Fold::Vertical(line.split('=').collect::<Vec<&str>>().get(1).expect("Invalid x fold").parse::<i64>().expect("Invalid x fold parse"))
                );
            } else if line.contains(",") {
                let coords = line.split(',').map(|coord| coord.parse::<i64>()).filter_map(Result::ok).collect::<Vec<i64>>();

                dots.insert((coords[0], coords[1]));
            }
        }

        Instructions {
            dots,
            folds,
        }
    }
}

impl Instructions {
    pub fn width(&self) -> i64 {
        self.dots.iter().fold(0, |acc, (x , _)| if *x > acc { *x } else { acc })
    }

    pub fn height(&self) -> i64 {
        self.dots.iter().fold(0, |acc, (_ , y)| if *y > acc { *y } else { acc })
    }
}

impl From<&Instructions> for Paper {
    fn from(instr: &Instructions) -> Self {
        let mut rows = vec![];

        for y in 0..=instr.height() {
            let mut row = vec![];

            for x in 0..=instr.width() {
                let has_coord = instr.dots.contains(&(x, y));

                row.push(has_coord);
            }

            rows.push(row);
        }

        Paper {
            rows,
        }
    }
}

impl Paper {
    pub fn fold(&self, fold: &Fold) -> Paper {
        let merge = |top: &Vec<Vec<bool>>, bottom: &Vec<Vec<bool>>| -> Vec<Vec<bool>> {bottom.iter().enumerate().map(|(y, row)| row.iter().enumerate().map(|(x, dot)|
            match (*dot, top.get(y).and_then(|t_row| t_row.get(x))) {
                (true, _) => true,
                (false, Some(&true)) => true,
                _ => false,
            }
        ).collect()).collect() };

        match fold {
            Fold::Horizontal(f_y) => {
                let top = self.rows.iter().skip(*f_y as usize + 1).rev().cloned().collect::<Vec<Vec<bool>>>();
                let bottom = self.rows.iter().take(*f_y as usize).cloned().collect::<Vec<Vec<bool>>>();

                Paper {
                    rows: merge(&top, &bottom),
                }
            },
            Fold::Vertical(f_x) => {
                let top = self.rows.iter().map(|row| row.iter().skip(*f_x as usize + 1).rev().cloned().collect()).collect::<Vec<Vec<bool>>>();
                let bottom = self.rows.iter().map(|row| row.iter().take(*f_x as usize).cloned().collect()).collect::<Vec<Vec<bool>>>();

                Paper {
                    rows: merge(&top, &bottom),
                }
            }
        }
    }

    pub fn count_visible_dots(&self) -> usize {
        self.rows.iter().map(|row| row.iter().filter(|dot| **dot).collect::<Vec<&bool>>()).flatten().collect::<Vec<&bool>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fold_first() {
        let instr = Instructions::from("6,10\n\
                                                    0,14\n\
                                                    9,10\n\
                                                    0,3\n\
                                                    10,4\n\
                                                    4,11\n\
                                                    6,0\n\
                                                    6,12\n\
                                                    4,1\n\
                                                    0,13\n\
                                                    10,12\n\
                                                    3,4\n\
                                                    3,0\n\
                                                    8,4\n\
                                                    1,10\n\
                                                    2,14\n\
                                                    8,10\n\
                                                    9,0\n\
                                                    \n\
                                                    fold along y=7\n\
                                                    fold along x=5");

        let paper = Paper::from(&instr);

        let folded = paper.fold(&instr.folds.first().unwrap());

        assert_eq!(folded.count_visible_dots(), 17);
    }
}