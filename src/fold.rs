use anyhow::{Result, Context, anyhow};
use std::collections::HashSet;
use std::fmt::Formatter;
use png_encode_mini::write_rgba_from_u8;

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
                folds.push(Fold::Horizontal(
                    line.split('=')
                        .collect::<Vec<&str>>()
                        .get(1)
                        .expect("Invalid y fold")
                        .parse::<i64>()
                        .expect("Invalid y fold parse"),
                ));
            } else if line.starts_with("fold along x=") {
                folds.push(Fold::Vertical(
                    line.split('=')
                        .collect::<Vec<&str>>()
                        .get(1)
                        .expect("Invalid x fold")
                        .parse::<i64>()
                        .expect("Invalid x fold parse"),
                ));
            } else if line.contains(',') {
                let coords = line
                    .split(',')
                    .map(|coord| coord.parse::<i64>())
                    .filter_map(Result::ok)
                    .collect::<Vec<i64>>();

                dots.insert((coords[0], coords[1]));
            }
        }

        Instructions { dots, folds }
    }
}

impl Instructions {
    pub fn width(&self) -> i64 {
        self.dots
            .iter()
            .fold(0, |acc, (x, _)| if *x > acc { *x } else { acc }) + 1
    }

    pub fn height(&self) -> i64 {
        self.dots
            .iter()
            .fold(0, |acc, (_, y)| if *y > acc { *y } else { acc }) + 1
    }
}

impl From<&Instructions> for Paper {
    fn from(instr: &Instructions) -> Self {
        let mut rows = vec![];

        for y in 0..instr.height() {
            let mut row = vec![];

            for x in 0..instr.width() {
                let has_coord = instr.dots.contains(&(x, y));

                row.push(has_coord);
            }

            rows.push(row);
        }

        Paper { rows }
    }
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|d| match *d {
                        true => "#".to_string(),
                        false => ".".to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

impl Paper {
    pub fn dump(&self, filename: &str, fold: Option<&Fold>) {
        let mut f = std::fs::File::create(filename).unwrap();

        let mut image: Vec<u8> = vec![];

        for y in (0..self.rows.len()).rev() {
            for x in 0..self.rows[0].len() {
                let mut fold_dot = false;

                if let Some(f) = fold {
                    match f {
                        Fold::Horizontal(f_y) => {
                            if *f_y == y as i64 {
                                fold_dot = true;
                            }
                        }
                        Fold::Vertical(f_x) => {
                            if *f_x == x as i64 {
                                fold_dot = true;
                            }
                        }
                    }
                }

                match (self.rows[y][x], fold_dot) {
                    (true, true) => {
                        image.push(0);
                        image.push(255);
                        image.push(0);
                        image.push(255);
                    },
                    (false, true) => {
                        image.push(255);
                        image.push(0);
                        image.push(0);
                        image.push(255);
                    },
                    (true, false) => {
                        image.push(255);
                        image.push(255);
                        image.push(255);
                        image.push(255);
                    },
                    (false, false) => {
                        image.push(0);
                        image.push(0);
                        image.push(0);
                        image.push(255);
                    }
                }
            }
        }

        write_rgba_from_u8(&mut f, &image, self.rows[0].len() as u32, self.rows.len() as u32).unwrap();
    }

    pub fn fold(&self, fold: &Fold) -> Result<Paper> {
        let merge = |top: &Vec<Vec<bool>>, bottom: &Vec<Vec<bool>>| -> Vec<Vec<bool>> {
            bottom
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(
                            |(x, dot)| matches!((*dot, top.get(y).and_then(|t_row| t_row.get(x))), (true, _) | (false, Some(&true))),
                        )
                        .collect()
                })
                .collect()
        };

        match fold {
            Fold::Horizontal(f_y) => {
                for dot in self.rows.get(*f_y as usize).context("Sanity check failure: Get horizontal seam")? {
                    if *dot {
                        return Err(anyhow!("Encountered dots in horizontal seam {}", f_y));
                    }
                }

                let mut top = self
                    .rows
                    .iter()
                    .skip(*f_y as usize + 1)
                    .take(*f_y as usize)
                    .cloned()
                    .collect::<Vec<Vec<bool>>>();

                let bottom = self
                    .rows
                    .iter()
                    .take(*f_y as usize)
                    .cloned()
                    .collect::<Vec<Vec<bool>>>();

                for _ in 0..(bottom.len() - top.len()) {
                    top.push(bottom[0].iter().map(|_| false).collect::<Vec<bool>>())
                }

                top.reverse();

                Ok(Paper {
                    rows: merge(&top, &bottom),
                })
            }
            Fold::Vertical(f_x) => {
                for row in &self.rows {
                    let dot = row.get(*f_x as usize).context("Sanity check failure: Get vertical seam")?;

                    if *dot {
                        return Err(anyhow!("Encountered dots in vertical seam {}", f_x))
                    }
                }


                let bottom = self
                    .rows
                    .iter()
                    .map(|row| row.iter().take(*f_x as usize).cloned().collect())
                    .collect::<Vec<Vec<bool>>>();
                let mut top = self
                    .rows
                    .iter()
                    .map(|row| {
                        let mut extended = row.iter().skip(*f_x as usize + 1).take(*f_x as usize).cloned().collect::<Vec<bool>>();

                        for _ in 0..(bottom[0].len() - extended.len()) {
                            extended.push(false);
                        }

                        extended
                    })
                    .collect::<Vec<Vec<bool>>>();

                top = top.iter()
                    .map(|row| row.iter().rev().cloned().collect())
                    .collect();

                Ok(Paper {
                    rows: merge(&top, &bottom),
                })
            }
        }
    }

    pub fn count_visible_dots(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.iter().filter(|dot| **dot).collect::<Vec<&bool>>())
            .flatten()
            .count()
    }
}

pub struct FoldablePaper {
    pub paper: Paper,
    instr: Instructions,
    fold_i: usize,
}

impl From<&Instructions> for FoldablePaper {
    fn from(instr: &Instructions) -> Self {
        FoldablePaper {
            paper: Paper::from(instr),
            instr: instr.clone(),
            fold_i: 0,
        }
    }
}

impl Iterator for FoldablePaper {
    type Item = Paper;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.paper.fold(self.instr.folds.get(self.fold_i)?).ok()?;

        self.fold_i += 1;
        self.paper = next.clone();

        Some(next)
    }

    fn count(self) -> usize where Self: Sized {
        self.instr.folds.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_horizontal() {
        let instr = Instructions::from("2,3\n\
                                                    2,7\n\
                                                    \n\
                                                    fold along y=5");

        let mut foldable = FoldablePaper::from(&instr);

        let folded = foldable.next().unwrap();

        assert!(folded.rows[3][2]);
        assert_eq!(folded.count_visible_dots(), 1);
    }

    #[test]
    fn test_fold_vertical() {
        let instr = Instructions::from("3,1\n\
                                                    7,1\n\
                                                    \n\
                                                    fold along x=5");

        let mut foldable = FoldablePaper::from(&instr);

        let folded = foldable.next().unwrap();

        assert!(folded.rows[1][3]);
        assert_eq!(folded.count_visible_dots(), 1);
    }

    #[test]
    fn test_count_fold_first() {
        let instr = Instructions::from(
            "6,10\n\
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
                                                    fold along x=5",
        );

        let mut foldable = FoldablePaper::from(&instr);

        assert_eq!(foldable.next().unwrap().count_visible_dots(), 17);
    }
}
