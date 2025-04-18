use std::fmt::Display;

struct Row(Vec<u8>);
struct Board(Vec<Row>);

impl Row {
    fn new() -> Self {
        Row(Vec::new())
    }

    fn of_size(size: usize) -> Self {
        let mut row = Vec::with_capacity(size);
        row.resize(size, 0);
        Row(row)
    }

    fn row_to_string(&self) -> String {
        let mut row_str = String::with_capacity(self.0.len());
        for cell in self.0.iter() {
            row_str.push_str(cell.to_string().as_str());
        }
        row_str
    }

    fn copy(&self) -> Self {
        let mut new_self = Self::of_size(self.0.len());
        for (i, cell) in self.0.iter().enumerate() {
            new_self.0[i] = *cell;
        }

        new_self
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ {} ]", self.row_to_string()))
    }
}

impl Board {
    fn new() -> Self {
        Board(Vec::new())
    }

    fn of_size(size: usize) -> Self {
        let mut board = Vec::with_capacity(size);
        board.resize_with(size, || Row::of_size(size));
        Board(board)
    }

    fn copy(&self) -> Self {
        let mut new_self = Self::of_size(self.0.len());
        for (i, row) in self.0.iter().enumerate() {
            new_self.0[i] = row.copy()
        }

        new_self
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();

        for (i, row) in self.0.iter().enumerate() {
            board_string.push_str(row.row_to_string().as_str());
            if i != self.0.len() - 1 {
                board_string.push('\n');
            }
        }

        f.write_str(&format!("[\n{}\n]", board_string))
    }
}

fn n_queens_problem(board_size: usize) -> Vec<Board> {
    let mut solutions = Vec::<Board>::new();

    if board_size == 0 {
        return solutions;
    }
    let board = Board::of_size(board_size);
    solve_nq_problem(0, 0, board_size, board, &mut solutions);

    // solutions.iter().for_each(|board| println!("{}", board));

    solutions
}

// Cell by Cell solution, place a queen in a cell X and then try to place a queen in the next
// available cell after exploring all paths starting from cell X, skip cell X and pick
// the next available cell to be the new Cell X. repeat
// Can also do column by column since picking a cell in a column auto disquallifies
// every other cell in that column (i.e) Pick a cell X the next column Y then try to place a
// queen in the next column after Y if possible repeat the steps for that column otherwise
// pick a different cell X in column Y
fn solve_nq_problem(
    start: usize,
    place_count: usize,
    size: usize,
    board: Board,
    solutions: &mut Vec<Board>,
) {
    if place_count == size {
        solutions.push(board);
        return;
    }

    for i in start..size * size {
        let (y, x) = get_cord(i, size);

        if (&board).0[y].0[x] != 0 {
            continue;
        }
        let new_board = get_board(i, &board);
        solve_nq_problem(i + 1, place_count + 1, size, new_board, solutions);
    }
}

fn get_board(index: usize, board: &Board) -> Board {
    let mut new_board = board.copy();
    let size = new_board.0.len();
    let (y, x) = get_cord(index, size);
    new_board.0[y].0[x] = 2;
    for k in 0..size {
        new_board.0[y].0[k] = if new_board.0[y].0[k] == 0 {
            1
        } else {
            new_board.0[y].0[k]
        };
        new_board.0[k].0[x] = if new_board.0[k].0[x] == 0 {
            1
        } else {
            new_board.0[k].0[x]
        };
    }

    let left_up_diag_steps = x.min(y);
    let right_up_diag_steps = (size - x - 1).min(y);
    let right_down_diag_steps = (size - x).min(size - y) - 1;
    let left_down_diag_steps = (x).min(size - y - 1);

    for k in 1..=left_up_diag_steps {
        let (n_y, n_x) = (y - k, x - k);
        new_board.0[n_y].0[n_x] = if new_board.0[n_y].0[n_x] == 0 {
            1
        } else {
            new_board.0[n_y].0[n_x]
        };
    }

    for k in 1..=left_down_diag_steps {
        let (n_y, n_x) = (y + k, x - k);
        new_board.0[n_y].0[n_x] = if new_board.0[n_y].0[n_x] == 0 {
            1
        } else {
            new_board.0[n_y].0[n_x]
        };
    }

    for k in 1..=right_up_diag_steps {
        let (n_y, n_x) = (y - k, x + k);
        new_board.0[n_y].0[n_x] = if new_board.0[n_y].0[n_x] == 0 {
            1
        } else {
            new_board.0[n_y].0[n_x]
        };
    }

    for k in 1..=right_down_diag_steps {
        let (n_y, n_x) = (y + k, x + k);
        new_board.0[n_y].0[n_x] = if new_board.0[n_y].0[n_x] == 0 {
            1
        } else {
            new_board.0[n_y].0[n_x]
        };
    }

    new_board
}

fn get_cord(index: usize, width: usize) -> (usize, usize) {
    (index / width, index % width)
}

fn n_queens_problem2(board_size: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::<Vec<usize>>::new();
    if board_size == 0 {
        return solutions;
    }

    let mut start_board = Vec::<Option<usize>>::with_capacity(board_size);
    start_board.resize(board_size, None);
    solve_nq_problem2(0, &mut start_board, &mut solutions);
    // solutions.iter().for_each(|x| println!("{:?}", x));
    solutions
}

fn solve_nq_problem2(
    curr_col: usize,
    board: &mut Vec<Option<usize>>,
    solutions: &mut Vec<Vec<usize>>,
) {
    if curr_col == board.len() {
        solutions.push(
            board
                .iter()
                .map(|x| match x {
                    Some(pos) => *pos,
                    _ => panic!("Error solution should not have None values"),
                })
                .collect(),
        );
        return;
    }
    for cell in 0..board.len() {
        if can_place(cell, curr_col, board) {
            board[curr_col] = Some(cell);
            solve_nq_problem2(curr_col + 1, board, solutions);
            board[curr_col] = None;
        }
    }
}

fn can_place(cell: usize, curr_col: usize, board: &Vec<Option<usize>>) -> bool {
    for k in 0..curr_col {
        let row = if let Some(val) = board[k] {
            val
        } else {
            panic!("Previous values where not set before check");
        };

        // on the same diagonal (they form a square)
        if curr_col - k == (cell.max(row) - cell.min(row)) {
            return false;
        }

        // on the same row
        if row == cell {
            return false;
        }
        // no need to check cols since they are always different
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_queens_problem() {
        let sol = n_queens_problem(4);
        println!("{} Solutions", sol.len())
    }

    #[test]
    fn test_n_queens_problem2() {
        let sol = n_queens_problem2(10);
        println!("{:?} Solutions", sol.len())
    }
}
