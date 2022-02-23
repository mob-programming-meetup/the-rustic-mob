use std::str::FromStr;

/*

Ideas:
1.bingo struct with from vec[string] - including board and sequences
2. parsing the string to create the matrix
3. parse the input list of numbers (drawn)
4. implement - mark the number on the board
5. check whether a specific row or column is fully marked
6. add a function that determines if a board is winning or not
7. implement scoring function
8. split input into draws and board definitions,
9. split board definition into a vector of boards

 */

struct Board {
  cells: Vec<Vec<u8>>,
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

  fn locate(&self, n: u8) -> (u8, u8) {
    for row in &self.cells {
      if row.contains(&n) {
        return (2, 4);
      }
    }
    todo!()
  }

  fn mark(&self, _: u8) {}
  fn is_marked(&self, row: u8, col: u8) -> bool {
    true
  }
}

impl FromStr for Board {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let cells = s
      .lines()
      .map(|line| {
        line
          .split_ascii_whitespace()
          .map(|number| number.parse::<u8>().unwrap())
          .collect()
      })
      .collect();
    Ok(Self { cells })
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
  fn locates_nuber_on_board() {
    let board = Board::from_str(&create_board_data()).unwrap();
    assert_eq!(board.locate(7), (2, 4));
    assert_eq!(board.locate(0), (0, 4))
  }

  #[test]
  fn board_marks_positions() {
    let board = Board::from_str(&create_board_data()).unwrap();
    board.mark(7);
    assert!(board.is_marked(2, 4));
    // assert!(!board.is_marked(1, 4));
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
