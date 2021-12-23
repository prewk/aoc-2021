use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug, Clone)]
pub struct BingoBoard {
    pub cols: [[u64; 5]; 5],
    pub rows: [[u64; 5]; 5],
}

impl From<&Vec<&str>> for BingoBoard {
    fn from(lines: &Vec<&str>) -> Self {
        let mut row_i = 0;
        let mut rows: [[u64; 5]; 5] = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let values: Vec<u64> = line
                .split(' ')
                .map(|n| n.parse::<u64>())
                .filter_map(Result::ok)
                .collect();

            rows[row_i][0] = *values.get(0).unwrap_or(&0);
            rows[row_i][1] = *values.get(1).unwrap_or(&0);
            rows[row_i][2] = *values.get(2).unwrap_or(&0);
            rows[row_i][3] = *values.get(3).unwrap_or(&0);
            rows[row_i][4] = *values.get(4).unwrap_or(&0);

            row_i += 1;
        }

        BingoBoard {
            rows,
            cols: [
                [rows[0][0], rows[1][0], rows[2][0], rows[3][0], rows[4][0]],
                [rows[0][1], rows[1][1], rows[2][1], rows[3][1], rows[4][1]],
                [rows[0][2], rows[1][2], rows[2][2], rows[3][2], rows[4][2]],
                [rows[0][3], rows[1][3], rows[2][3], rows[3][3], rows[4][3]],
                [rows[0][4], rows[1][4], rows[2][4], rows[3][4], rows[4][4]],
            ],
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Play {
    board: BingoBoard,
    marked: HashMap<u64, bool>,
}

impl From<BingoBoard> for Play {
    fn from(board: BingoBoard) -> Self {
        let mut marked = HashMap::new();

        for row in &board.rows {
            marked.insert(row[0], false);
            marked.insert(row[1], false);
            marked.insert(row[2], false);
            marked.insert(row[3], false);
            marked.insert(row[4], false);
        }

        Play { board, marked }
    }
}

impl Play {
    pub fn draw(&mut self, drawn: &u64) {
        if self.marked.contains_key(drawn) {
            self.marked.insert(*drawn, true);
        }
    }

    pub fn check(&self) -> bool {
        for row in self.board.rows {
            let checked = row.map(|n| *self.marked.get(&n).unwrap_or(&false));

            if checked[0] && checked[1] && checked[2] && checked[3] && checked[4] {
                return true;
            }
        }

        for col in self.board.cols {
            let checked = col.map(|n| *self.marked.get(&n).unwrap_or(&false));

            if checked[0] && checked[1] && checked[2] && checked[3] && checked[4] {
                return true;
            }
        }

        false
    }

    pub fn get_unmarked_sum(&self) -> u64 {
        self.marked
            .iter()
            .filter(|(_k, v)| !(**v))
            .map(|(k, _v)| k)
            .sum()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Bingo {
    pub draws: Vec<u64>,
    pub boards: Vec<BingoBoard>,
}

impl From<&str> for Bingo {
    fn from(input: &str) -> Self {
        let mut board_data: Vec<&str> = vec![];
        let mut boards: Vec<BingoBoard> = vec![];

        let draws: Vec<u64> = input
            .lines()
            .next()
            .expect("First line must exist")
            .split(',')
            .map(|n| n.parse::<u64>())
            .filter_map(Result::ok)
            .collect();

        for line in input.lines().skip(1) {
            board_data.push(line);

            if board_data.len() == 6 {
                boards.push(BingoBoard::from(&board_data));
                board_data.clear();
            }
        }

        Bingo { draws, boards }
    }
}

impl Bingo {
    pub fn play(&mut self) -> Option<u64> {
        let mut plays: Vec<Play> = self
            .boards
            .iter()
            .map(|board| Play::from(board.clone()))
            .collect();

        for drawn in &self.draws {
            for play in plays.iter_mut() {
                play.draw(drawn);
            }

            for play in &plays {
                if play.check() {
                    return Some(play.get_unmarked_sum() * drawn);
                }
            }
        }

        None
    }

    pub fn play_last(&mut self) -> Option<u64> {
        let mut plays: Vec<Play> = self
            .boards
            .iter()
            .map(|board| Play::from(board.clone()))
            .collect();

        let mut winners = HashSet::new();

        for drawn in &self.draws {
            for play in plays.iter_mut() {
                play.draw(drawn);
            }

            let mut play_i = 0;
            for play in &plays {
                if play.check() && !winners.contains(&play_i) {
                    winners.insert(play_i);

                    if winners.len() == plays.len() {
                        return Some(play.get_unmarked_sum() * drawn);
                    }
                }

                play_i += 1;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play() {
        let mut bingo = Bingo::from(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
                                            \n\
                                            22 13 17 11  0\n\
                                             8  2 23  4 24\n\
                                            21  9 14 16  7\n\
                                             6 10  3 18  5\n\
                                             1 12 20 15 19\n\
                                            \n\
                                             3 15  0  2 22\n\
                                             9 18 13 17  5\n\
                                            19  8  7 25 23\n\
                                            20 11 10 24  4\n\
                                            14 21 16 12  6\n\
                                            \n\
                                            14 21 17 24  4\n\
                                            10 16 15  9 19\n\
                                            18  8 23 26 20\n\
                                            22 11 13  6  5\n\
                                             2  0 12  3  7",
        );

        assert_eq!(bingo.play(), Some(4512));
    }

    #[test]
    fn test_play_last() {
        let mut bingo = Bingo::from(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
                                            \n\
                                            22 13 17 11  0\n\
                                             8  2 23  4 24\n\
                                            21  9 14 16  7\n\
                                             6 10  3 18  5\n\
                                             1 12 20 15 19\n\
                                            \n\
                                             3 15  0  2 22\n\
                                             9 18 13 17  5\n\
                                            19  8  7 25 23\n\
                                            20 11 10 24  4\n\
                                            14 21 16 12  6\n\
                                            \n\
                                            14 21 17 24  4\n\
                                            10 16 15  9 19\n\
                                            18  8 23 26 20\n\
                                            22 11 13  6  5\n\
                                             2  0 12  3  7",
        );

        assert_eq!(bingo.play_last(), Some(1924));
    }
}
