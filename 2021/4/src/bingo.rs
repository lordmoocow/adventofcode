use std::str::FromStr;

const BOARD_SIZE: usize = 5;

#[derive(Default, Debug, Copy, Clone)]
pub struct Board {
    numbers: [[u8; BOARD_SIZE]; BOARD_SIZE],
    marks: [[bool; BOARD_SIZE]; BOARD_SIZE],
    complete: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBoardErr;

impl FromStr for Board {
    type Err = ParseBoardErr;
    fn from_str(data: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut board = Board::default();
        for (x, row) in data.split('\n').enumerate() {
            for (y, n) in row.split(' ').filter_map(|x| x.parse().ok()).enumerate() {
                board.numbers[x][y] = n;
            }
        }
        Ok(board)
    }
}

impl Board {
    pub fn mark(&mut self, number: u8) {
        let mut count_row;
        let mut count_col;
        // iterate rows/cols to find a matching number
        // while we iterate, count the marks to see if we are complete
        for (x, row) in self.numbers.iter().enumerate() {
            count_row = 0;
            count_col = 0;
            for (y, n) in row.iter().enumerate() {
                if n == &number {
                    self.marks[x][y] = true;
                }
                
                // count marks as we iterate the row
                if self.marks[x][y] {
                    count_row += 1;
                }
                // count marks in cols
                if self.marks[y][x] { 
                    count_col += 1;
                }
            }

            // flag board as complete if we have either a full row or col
            if count_row == BOARD_SIZE || count_col == BOARD_SIZE {
                self.complete = number;
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        self.complete > 0
    }

    pub fn get_score(&self) -> usize {
        let mut score = 0;
        for (x, row) in self.numbers.iter().enumerate() {
            for (y, n) in row.iter().enumerate() {
                if !self.marks[x][y] {
                    score += *n as usize;
                }
            }
        }
        score * self.complete as usize
    }
}

#[derive(Default, Debug)]
pub struct System {
    draw_sequence: Vec<u8>,
    boards: Vec<Board>,
}

impl System {
    pub fn set_draw_sequence(&mut self, csv: &str) {
        self.draw_sequence = csv.split(',').filter_map(|x| x.parse().ok()).collect();
    }

    pub fn add_board(&mut self, board: Board) {
        self.boards.push(board)
    }

    pub fn run(&mut self) -> Option<Board> {
        for draw in self.draw_sequence.iter() {
            for board in self.boards.iter_mut() {
                board.mark(*draw);
                if board.is_complete() {
                    return Some(*board);
                }
            }
        }
        None
    }
}
