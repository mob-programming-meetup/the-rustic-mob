use std::str::FromStr;

pub fn run(input: &str) {
  todo!()
}

// Ideas:
// 01. bingo struct with from vec[string] - including board and sequences
// 02. parsing the string to create the matrix
// 03. parse the input list of numbers (drawn)
// 04. implement - mark the number on the board
// 05. check whether a specific row or column is fully marked
// 06. add a function that determines if a board is winning or not
//  a. possible impl could check each row & col for a win
//  b. could be made more efficient
// 07. implement scoring function
// 08. split input into draws and board definitions,
// 09. split board definition into a vector of boards
// 10. keep track of marked cells in a matrix of bool
// 11. to represent numbers with the times-table

struct Board {
  cells: Vec<Vec<u8>>,
  marked: Vec<Vec<bool>>,
}

impl Board {
  fn contains(&self, n: u8) -> bool {
    for row in &self.cells {
      if row.contains(&n) {
        return true;
      }
    }
    false
  }

  fn locate(&self, n: u8) -> Option<(usize, usize)> {
    for (row_id, row) in self.cells.iter().enumerate() {
      for (col_id, &cell) in row.iter().enumerate() {
        if cell == n {
          return Some((row_id, col_id));
        }
      }
    }
    None
  }

  fn mark(&mut self, n: u8) {
    if let Some((row, col)) = self.locate(n) {
      self.marked[row][col] = true;
    }
  }

  fn is_marked(&self, row: usize, col: usize) -> bool {
    self.marked[row][col]
  }

  fn row_is_marked(&self, row: usize) -> bool {
    self.marked[row].iter().all(|&cell| cell)
  }

  fn col_is_marked(&self, col: usize) -> bool {
    self.marked.iter().all(|row| row[col])
  }
}

impl FromStr for Board {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let cells: Vec<Vec<u8>> = s
      .lines()
      .map(|line| {
        line
          .split_ascii_whitespace()
          .map(|number| number.parse::<u8>().unwrap())
          .collect()
      })
      .collect();
    let marked = vec![vec![false; cells[0].len()]; cells.len()];
    Ok(Self { cells, marked })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn given_input_create_board() {
    let board = Board::from_str(&create_board_data()).unwrap();
    assert!(board.contains(7));
    assert!(!board.contains(77));
    assert!(board.contains(9));
  }

  #[test]
  fn locates_number_on_board() {
    let board = Board::from_str(&create_board_data()).unwrap();
    assert_eq!(board.locate(7), Some((2, 4)));
    assert_eq!(board.locate(0), Some((0, 4)));
    assert_eq!(board.locate(8), Some((1, 0)));
    assert_eq!(board.locate(99), None);
  }

  #[test]
  fn board_marks_positions() {
    let mut board = Board::from_str(&create_board_data()).unwrap();
    board.mark(7);
    assert!(board.is_marked(2, 4));
    assert!(!board.is_marked(1, 4));
  }

  #[test]
  fn board_row_is_marked() {
    let mut board = Board::from_str(&create_board_data()).unwrap();
    board.mark(8);
    board.mark(2);
    board.mark(23);
    board.mark(4);
    board.mark(24);
    assert!(board.row_is_marked(1));
    assert!(!board.row_is_marked(0));
  }

  #[test]
  fn board_column_is_marked() {
    let mut board = Board::from_str(&create_board_data()).unwrap();
    board.mark(13);
    board.mark(2);
    board.mark(9);
    board.mark(10);
    board.mark(12);
    assert!(board.col_is_marked(1));
    assert!(!board.col_is_marked(0));
  }

  //TODO: Change this to a static string
  fn create_board_data() -> String {
    String::from(indoc! {"
      22 13 17 11  0
       8  2 23  4 24
      21  9 14 16  7
       6 10  3 18  5
       1 12 20 15 91
    "})
  }
}
